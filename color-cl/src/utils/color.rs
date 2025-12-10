use regex::Regex;
use std::fmt::Display;

pub struct RGBA(u8, u8, u8, f64);

pub struct Color {
  pub rgba: RGBA,
  pub hex: String,
}
impl Color {
  pub fn parse(input: &str) -> Result<Self, String> {
    let input = input.trim();
    if input.to_lowercase().starts_with("rgb") {
      let re = Regex::new(r"^rgba?\s*\(\s*(\d+)\s*,\s*(\d+)\s*,\s*(\d+)(?:\s*,\s*([\d.]+))?\s*\)$")
        .unwrap();
      let caps = re.captures(input).ok_or("fail to captures")?;
      let r = caps
        .get(1)
        .ok_or("fail to get red")?
        .as_str()
        .parse::<u8>()
        .or(Err("fail to parse red"))?;
      let g = caps
        .get(2)
        .ok_or("fail to get green")?
        .as_str()
        .parse::<u8>()
        .or(Err("fail to parse green"))?;
      let b = caps
        .get(3)
        .ok_or("fail to get blue")?
        .as_str()
        .parse::<u8>()
        .or(Err("fail to parse blue"))?;
      let a = caps
        .get(4)
        .and_then(|m| m.as_str().parse::<f64>().ok())
        .unwrap_or(1.0);
      return Ok(Self::from_rgba(RGBA(r, g, b, a)));
    }
    Self::from_hex(input)
  }

  fn from_hex(hex: &str) -> Result<Self, String> {
    let mut hex = String::from(hex.trim_start_matches("#"));
    if hex.len() == 3 || hex.len() == 4 {
      let mut new_hex = String::new();
      for char in hex.chars() {
        new_hex.push(char);
        new_hex.push(char);
      }
      hex = new_hex;
    }
    let red = hex
      .get(0..2)
      .ok_or_else(|| format!("fail to get red {}", hex))
      .and_then(|v| u8::from_str_radix(v, 16).map_err(|_| format!("fail to parse red {}", hex)))?;
    let green = hex
      .get(2..4)
      .ok_or_else(|| format!("fail to get green {}", hex))
      .and_then(|v| {
        u8::from_str_radix(v, 16).map_err(|_| format!("fail to parse green {}", hex))
      })?;
    let blue = hex
      .get(4..6)
      .ok_or_else(|| format!("fail to get blue {}", hex))
      .and_then(|v| u8::from_str_radix(v, 16).map_err(|_| format!("fail to parse blue {}", hex)))?;
    let alpha = if hex.len() == 8 {
      hex
        .get(6..8)
        .ok_or_else(|| format!("fail to get alpha {}", hex))
        .and_then(|v| u8::from_str_radix(v, 16).map_err(|_| format!("fail to parse alpha {}", hex)))
        .and_then(|v| Ok((v as f64) / 255.0))?
    } else {
      1.0
    };
    Ok(Self {
      rgba: RGBA(red, green, blue, alpha),
      hex: hex.to_owned(),
    })
  }

  fn from_rgba(rgba: RGBA) -> Self {
    let mut hex_str = String::from(format!("{:02X}", rgba.0));
    hex_str.push_str(&format!("{:02X}", rgba.1));
    hex_str.push_str(&format!("{:02X}", rgba.2));

    if rgba.3 < 1.0 {
      hex_str.push_str(&format!("{:02X}", (rgba.3 * 255.0).round() as u8));
    }

    Self { rgba, hex: hex_str }
  }
}
impl Display for Color {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let rgba = if self.rgba.3 == 1.0 {
      format!("rgb({},{},{})", self.rgba.0, self.rgba.1, self.rgba.2)
    } else {
      format!("rgba({},{},{},{:.2})", self.rgba.0, self.rgba.1, self.rgba.2, self.rgba.3)
    };
    write!(f, "{} #{}", rgba, self.hex)
  }
}
