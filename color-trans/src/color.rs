use std::fmt::Display;

#[derive(Debug)]
#[derive(Default)]
pub struct RGBA(u8, u8, u8, f64);
impl PartialEq for RGBA {
  fn eq(&self, other: &Self) -> bool {
    self.0 == other.0 && self.1 == other.1 && self.2 == other.2 && self.3 == other.3
  }
}
impl Display for RGBA {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "rgba({},{},{},{})", self.0, self.1, self.2, self.3)
  }
}
impl TryFrom<&str> for RGBA {
    
    type Error = String;
    
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        hex2rgba(value)
    }
}
impl TryFrom<&String> for RGBA {
    
    type Error = String;
    
    fn try_from(value: &String) -> Result<Self, Self::Error> {
        hex2rgba(value)
    }
}
impl TryFrom<(u8, u8, u8, f64)> for RGBA {
    
    type Error = String;
    
    fn try_from((r, g, b, a): (u8, u8, u8, f64)) -> Result<Self, Self::Error> {
        Ok(Self(r, g, b, a))
    }
}

pub struct Color {
  pub rgba: RGBA,
  pub hex: String,
}

fn rgba2hex(rgba: &RGBA) -> String {
  let mut hex_str = String::from(
    format!("{:02X}", rgba.0)
  );
  hex_str.push_str(&format!("{:02X}", rgba.1));
  hex_str.push_str(&format!("{:02X}", rgba.2));

  if rgba.3 < 1.0 {
    hex_str.push_str(&format!("{:02X}", (rgba.3 * 255.0).round() as u8));
  }

  hex_str
}

fn hex2rgba(hex: &str) -> Result<RGBA, String> {
  let red = hex.get(0..2)
    .ok_or_else(|| format!("fail to get red {}", hex))
    .and_then(|v| {
      u8::from_str_radix(v, 16).map_err(|_| format!("fail to parse red {}", hex))
    })?;
  let green = hex.get(2..4)
    .ok_or_else(|| format!("fail to get green {}", hex))
    .and_then(|v| {
      u8::from_str_radix(v, 16).map_err(|_| format!("fail to parse green {}", hex))
    })?;
  let blue = hex.get(4..6)
    .ok_or_else(|| format!("fail to get blue {}", hex))
    .and_then(|v| {
      u8::from_str_radix(v, 16).map_err(|_| format!("fail to parse blue {}", hex))
    })?;
  let alpha = if hex.len() == 8 {
    hex.get(6..8)
    .ok_or_else(|| format!("fail to get alpha {}", hex))
    .and_then(|v| {
      u8::from_str_radix(v, 16).map_err(|_| format!("fail to parse alpha {}", hex))
    })
    .and_then(|v| {
      Ok((v as f64) / 255.0)
    })?
  } else {
    1.0
  };

  Ok(RGBA(red, green, blue, alpha))
}

impl Color {

  pub fn from_rgba(rgba: RGBA) -> Self {
    let hex = rgba2hex(&rgba);
    Self{
      rgba,
      hex,
    }
  }

  pub fn update<T>(&mut self, color: T) -> Result<(), T::Error>
    where
      T: TryInto<RGBA>
  {
    let rgba = color.try_into()?;
    let hex = rgba2hex(&rgba); 
    self.rgba = rgba;
    self.hex = hex;
    Ok(())
  }

}
impl Default for Color {
  fn default() -> Self {
    Color::from_rgba(RGBA::default())
  }
}

#[cfg(test)]
mod tests {
  use crate::color::{Color, RGBA};


  #[test]
  fn color_from() {
    let color = Color::from_rgba(RGBA(0, 0, 0, 1.0));
    assert_eq!(color.hex, "000000");
  }

}
