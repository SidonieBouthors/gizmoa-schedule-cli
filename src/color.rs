use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug)]
pub struct HexColor {
  pub r: u8,
  pub g: u8,
  pub b: u8,
  pub a: u8,
}

impl HexColor {
  /// Create from hex string
  pub fn from_hex(hex: &str) -> Result<Self, String> {
    let hex = hex.trim_start_matches('#');
    let r = u8::from_str_radix(&hex[0..2], 16).map_err(|_| "Invalid red value")?;
    let g = u8::from_str_radix(&hex[2..4], 16).map_err(|_| "Invalid green value")?;
    let b = u8::from_str_radix(&hex[4..6], 16).map_err(|_| "Invalid blue value")?;
    let a = if hex.len() == 8 {
      u8::from_str_radix(&hex[6..8], 16).map_err(|_| "Invalid alpha value")?
    } else {
      255
    };
    Ok(HexColor { r, g, b, a })
  }

  // Convert to hex string
  pub fn to_hex(&self) -> String {
    format!("#{:02X}{:02X}{:02X}{:02X}", self.r, self.g, self.b, self.a)
  }

  pub fn to_tuple(&self) -> (u8, u8, u8, u8) {
    (self.r, self.g, self.b, self.a)
  }
}

impl Serialize for HexColor {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_str(&self.to_hex())
  }
}

impl<'de> Deserialize<'de> for HexColor {
  fn deserialize<D>(deserializer: D) -> Result<HexColor, D::Error>
  where
    D: Deserializer<'de>,
  {
    let s = String::deserialize(deserializer)?;
    HexColor::from_hex(&s).map_err(serde::de::Error::custom)
  }
}
