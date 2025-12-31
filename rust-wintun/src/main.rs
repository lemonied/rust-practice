use etherparse::PacketHeaders;
use httparse;
use std::io::{self};
use std::process::Command;
use std::str;
use std::sync::Arc;
use std::sync::Once;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use wintun::Adapter;

static CLEANED: AtomicBool = AtomicBool::new(false);
static PANIC_HOOK_SET: Once = Once::new();

fn run_cmd(cmd: &str, args: &[&str]) -> io::Result<()> {
  let output = Command::new(cmd).args(args).output()?;
  if output.status.success() {
    Ok(())
  } else {
    let mut err = String::new();
    if !output.stderr.is_empty() {
      if let Ok(s) = String::from_utf8(output.stderr.clone()) {
        err = s;
      }
    }
    Err(io::Error::new(
      io::ErrorKind::Other,
      format!(
        "命令失败: {} {:?}\nstdout: {}\nstderr: {}",
        cmd,
        args,
        String::from_utf8_lossy(&output.stdout),
        err
      ),
    ))
  }
}

fn run_cmd_capture(cmd: &str, args: &[&str]) -> io::Result<String> {
  let output = Command::new(cmd).args(args).output()?;
  if output.status.success() {
    let s = String::from_utf8_lossy(&output.stdout).trim().to_string();
    Ok(s)
  } else {
    let stderr = String::from_utf8_lossy(&output.stderr);
    Err(io::Error::new(
      io::ErrorKind::Other,
      format!("命令失败: {} {:?}\nstderr: {}", cmd, args, stderr),
    ))
  }
}

fn get_interface_index(tun_name: &str) -> io::Result<u32> {
  let ps_cmd = format!(
    "(Get-NetAdapter -Name '{}' -ErrorAction Stop).IfIndex",
    tun_name.replace('\'', "''")
  );
  match run_cmd_capture("powershell", &["-NoProfile", "-Command", &ps_cmd]) {
    Ok(out) => {
      let trimmed = out.trim();
      if trimmed.is_empty() {
        Err(io::Error::new(
          io::ErrorKind::Other,
          "PowerShell 返回空，如果适配器不存在会发生此问题",
        ))
      } else {
        match trimmed.parse::<u32>() {
          Ok(idx) => Ok(idx),
          Err(_) => Err(io::Error::new(
            io::ErrorKind::Other,
            format!("无法解析接口索引: {}", trimmed),
          )),
        }
      }
    }
    Err(e) => {
      eprintln!(
        "Warning: 使用 PowerShell 获取接口索引失败: {}，改为解析 netsh 输出",
        e
      );
      let out = run_cmd_capture("netsh", &["interface", "ipv4", "show", "interfaces"])?;
      for line in out.lines() {
        if line.contains(tun_name) {
          let parts: Vec<&str> = line.split_whitespace().collect();
          if !parts.is_empty() {
            if let Ok(idx) = parts[0].parse::<u32>() {
              return Ok(idx);
            }
          }
        }
      }
      Err(io::Error::new(
        io::ErrorKind::Other,
        "无法从 netsh 输出解析接口索引，确认接口是否已创建并且名称完全匹配",
      ))
    }
  }
}

fn enable_tun(tun_name: &str, tun_ip: &str, tun_mask: &str) -> io::Result<u32> {
  println!(
    "[*] 配置虚拟网卡 IP: {} -> {}/{}",
    tun_name, tun_ip, tun_mask
  );

  run_cmd(
    "netsh",
    &[
      "interface",
      "ipv4",
      "set",
      "address",
      &format!("name={}", tun_name),
      "source=static",
      &format!("addr={}", tun_ip),
      &format!("mask={}", tun_mask),
      "gateway=none",
    ],
  )?;

  let if_index = get_interface_index(tun_name)?;
  println!("[*] 接口索引: {}", if_index);

  println!("[*] 添加默认路由到虚拟网卡 (绑定 ifIndex)");
  run_cmd(
    "route",
    &[
      "add",
      "0.0.0.0",
      "mask",
      "0.0.0.0",
      "0.0.0.0",
      "metric",
      "1",
      "if",
      &if_index.to_string(),
    ],
  )?;

  Ok(if_index)
}

fn restore_net(phy_name: &str, tun_if_index: Option<u32>) -> io::Result<()> {
  if CLEANED.swap(true, Ordering::SeqCst) {
    return Ok(());
  }

  if let Some(idx) = tun_if_index {
    println!("[*] 删除默认路由到虚拟网卡 (ifIndex={})", idx);
    let _ = run_cmd(
      "route",
      &[
        "delete",
        "0.0.0.0",
        "mask",
        "0.0.0.0",
        "0.0.0.0",
        "if",
        &idx.to_string(),
      ],
    );
  } else {
    println!("[*] 未提供 tun 接口索引，尝试删除所有匹配的默认路由（可能会删除非本程序添加的路由）");
    let _ = run_cmd("route", &["delete", "0.0.0.0"])?;
  }

  println!("[*] 恢复物理网卡 DHCP: {}", phy_name);
  let _ = run_cmd(
    "netsh",
    &[
      "interface",
      "ipv4",
      "set",
      "address",
      &format!("name={}", phy_name),
      "source=dhcp",
    ],
  );
  let _ = run_cmd(
    "netsh",
    &[
      "interface",
      "ipv4",
      "set",
      "dns",
      &format!("name={}", phy_name),
      "source=dhcp",
    ],
  );
  let _ = run_cmd("ipconfig", &["/renew"]);

  println!("[*] 恢复完成");
  Ok(())
}

fn install_panic_hook(phy_name: String, tun_if_index: Arc<std::sync::Mutex<Option<u32>>>) {
  PANIC_HOOK_SET.call_once(|| {
    let prev_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
      eprintln!("[!] 程序 panic，尝试恢复网络...");
      let idx = *tun_if_index.lock().unwrap();
      if let Err(e) = restore_net(&phy_name, idx) {
        eprintln!("[!] 恢复网络失败: {}", e);
      }
      prev_hook(info);
    }));
  });
}

fn looks_like_http_port(p: u16) -> bool {
  matches!(p, 80 | 8080 | 8000 | 8008 | 8888)
}

fn try_parse_http_from_bytes(bytes: &[u8]) {
  let mut headers = [httparse::EMPTY_HEADER; 64];
  let mut req = httparse::Request::new(&mut headers);
  match req.parse(bytes) {
    Ok(httparse::Status::Complete(_)) => {
      println!(
        "  [HTTP Request] method={:?} path={:?} version={:?}",
        req.method, req.path, req.version
      );
      for h in req.headers.iter() {
        if let Ok(v) = str::from_utf8(h.value) {
          println!("    {}: {}", h.name, v);
        }
      }
      return;
    }
    Ok(httparse::Status::Partial) => {}
    Err(e) => {
      eprintln!("  [httparse request error] {:?}", e);
    }
  }

  let mut res_headers = [httparse::EMPTY_HEADER; 64];
  let mut res = httparse::Response::new(&mut res_headers);
  match res.parse(bytes) {
    Ok(httparse::Status::Complete(_)) => {
      println!(
        "  [HTTP Response] code={:?} reason={:?} version={:?}",
        res.code, res.reason, res.version
      );
      for h in res.headers.iter() {
        if let Ok(v) = str::from_utf8(h.value) {
          println!("    {}: {}", h.name, v);
        }
      }
      return;
    }
    Ok(httparse::Status::Partial) => {
      println!("  [HTTP] header incomplete (need reassembly)");
    }
    Err(e) => {
      eprintln!("  [httparse response error] {:?}", e);
    }
  }
}

fn main() -> io::Result<()> {
  let tun_name = "RustTun";
  let tun_ip = "10.0.0.1";
  let tun_mask = "255.255.255.0";
  let phy_name = "Ethernet";

  let tun_if_index = match enable_tun(tun_name, tun_ip, tun_mask) {
    Ok(idx) => Some(idx),
    Err(e) => {
      eprintln!("[!] 设置虚拟网卡/路由失败: {}", e);
      eprintln!("[!] 请以管理员身份运行程序并确认虚拟网卡存在。程序继续但不会修改路由。");
      None
    }
  };

  let tun_if_index_shared = Arc::new(std::sync::Mutex::new(tun_if_index));
  install_panic_hook(phy_name.to_string(), tun_if_index_shared.clone());

  {
    let phy_name_clone = phy_name.to_string();
    let tun_if_index_clone = tun_if_index_shared.clone();
    ctrlc::set_handler(move || {
      println!("[*] 捕获到 Ctrl+C，正在恢复网络...");
      let idx = *tun_if_index_clone.lock().unwrap();
      if let Err(e) = restore_net(&phy_name_clone, idx) {
        eprintln!("[!] 恢复网络失败: {}", e);
      }
      std::process::exit(0);
    })
    .expect("无法设置 Ctrl+C 处理");
  }

  let wintun = unsafe { wintun::load_from_path("wintun.dll") }.or_else(|_| {
    Err(io::Error::new(
      io::ErrorKind::Other,
      "无法加载 wintun.dll，确保 wintun 已安装且 wintun.dll 在 PATH 或与可执行文件同目录",
    ))
  })?;

  let adapter = match Adapter::open(&wintun, tun_name) {
    Ok(a) => a,
    Err(_) => {
      println!("[*] 找不到适配器，正在创建: {}", tun_name);
      wintun::Adapter::create(&wintun, tun_name, "Rust Capture Tunnel", None)
        .expect("创建 wintun 适配器失败")
    }
  };

  let mut session = adapter
    .start_session(0x200000) // 2MB buffer
    .expect("启动 Wintun Session 失败");

  println!("[*] 开始拦截流量 (Ctrl+C 退出)");

  loop {
    let packet = match session.receive_blocking() {
      Ok(pkt) => pkt,
      Err(e) => {
        eprintln!("[!] 接收数据失败: {}，短暂休眠后重试...", e);
        std::thread::sleep(Duration::from_millis(200));
        continue;
      }
    };
    let bytes = packet.bytes();

    match PacketHeaders::from_ip_slice(bytes) {
      Ok(headers) => {
        // 计算 IP 头长度（以字节为单位）
        let ip_header_len = match bytes.get(0) {
          Some(b0) => {
            let version = b0 >> 4;
            match version {
              4 => {
                // IPv4: IHL field (low nibble) * 4
                let ihl = (b0 & 0x0f) as usize;
                ihl * 4
              }
              6 => {
                // IPv6 fixed header
                40usize
              }
              _ => 0usize,
            }
          }
          None => 0usize,
        };

        // 计算传输层头长度（TCP variable, UDP fixed）
        let transport_header_len = match headers.transport {
          Some(etherparse::TransportHeader::Tcp(ref tcp)) => {
            // TCP data offset is in the first byte of TCP offset/flags (offset is high 4 bits of byte 12)
            // safer to read directly from bytes at tcp header offset if available
            let tcp_offset = ip_header_len;
            if bytes.len() > tcp_offset + 12 {
              let data_offset_byte = bytes[tcp_offset + 12];
              let data_offset = ((data_offset_byte >> 4) as usize) * 4;
              if data_offset >= 20 { data_offset } else { 20 }
            } else {
              20usize
            }
          }
          Some(etherparse::TransportHeader::Udp(_udp)) => 8usize,
          _ => 0usize,
        };

        let payload_offset = ip_header_len.saturating_add(transport_header_len);
        let payload_slice: &[u8] = if payload_offset <= bytes.len() {
          &bytes[payload_offset..]
        } else {
          &[]
        };

        if let Some(ip) = headers.ip {
          match ip {
            etherparse::IpHeader::Version4(ipv4, _) => {
              println!(
                "IPv4 {} -> {}, ttl={}, protocol={}, payload_len={}",
                ipv4.source_addr, ipv4.destination_addr, ipv4.ttl, ipv4.protocol, ipv4.payload_len
              );
            }
            etherparse::IpHeader::Version6(ipv6, _) => {
              println!(
                "IPv6 {:?} -> {:?}, hop_limit={}, next_header={}",
                ipv6.source, ipv6.destination, ipv6.hop_limit, ipv6.next_header
              );
            }
          }
        }

        if let Some(trans) = headers.transport {
          match trans {
            etherparse::TransportHeader::Tcp(tcp) => {
              println!(
                "TCP {} -> {}, seq={}, ack={}, window={}",
                tcp.source_port,
                tcp.destination_port,
                tcp.sequence_number,
                tcp.acknowledgment_number,
                tcp.window_size
              );
              // flags may or may not exist depending on etherparse version; attempt to print common ones if present
              println!(
                "  flags: syn={} fin={} rst={} psh={} ack={}",
                tcp.syn, tcp.fin, tcp.rst, tcp.psh, tcp.ack
              );

              if !payload_slice.is_empty() {
                let to_take = std::cmp::min(payload_slice.len(), 4096);
                let payload_owned = payload_slice[..to_take].to_vec();
                drop(packet);

                let preview = &payload_owned[..std::cmp::min(payload_owned.len(), 128)];
                println!(
                  "  payload (first {} bytes hex): {:02x?}",
                  preview.len(),
                  preview
                );
                if let Ok(s) = std::str::from_utf8(preview) {
                  println!("  payload(as utf8): {}", s);
                }

                if looks_like_http_port(tcp.source_port)
                  || looks_like_http_port(tcp.destination_port)
                {
                  try_parse_http_from_bytes(&payload_owned);
                }

                continue;
              } else {
                drop(packet);
                continue;
              }
            }
            etherparse::TransportHeader::Udp(udp) => {
              println!(
                "UDP {} -> {}, length={}",
                udp.source_port, udp.destination_port, udp.length
              );
              if !payload_slice.is_empty() {
                let to_take = std::cmp::min(payload_slice.len(), 2048);
                let payload_owned = payload_slice[..to_take].to_vec();
                drop(packet);

                let preview = &payload_owned[..std::cmp::min(payload_owned.len(), 128)];
                println!(
                  "  payload (first {} bytes hex): {:02x?}",
                  preview.len(),
                  preview
                );
                if let Ok(s) = std::str::from_utf8(preview) {
                  println!("  payload(as utf8): {}", s);
                }

                if udp.source_port == 53 || udp.destination_port == 53 {
                  println!("  [UDP 53] (DNS) payload len {}", payload_owned.len());
                }

                continue;
              } else {
                drop(packet);
                continue;
              }
            }
            _ => {
              println!("Other transport");
              drop(packet);
              continue;
            }
          }
        } else {
          drop(packet);
          continue;
        }
      }
      Err(e) => {
        println!("解析失败: {:?}", e);
        drop(packet);
        continue;
      }
    }
  }
}
