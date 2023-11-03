//! Contains functions to convert colors from one format to another

/// Converts an array of pixel color to it's hexadecimal representation
/// # Example
/// ```rust
/// # use fast_qr::convert::rgba2hex;
/// let color = [0, 0, 0, 255];
/// assert_eq!(&rgba2hex(color), "#000000");
/// ```
#[must_use]
pub fn rgba2hex(color: [u8; 4]) -> String {
    let mut hex = String::with_capacity(9);

    hex.push('#');
    hex.push_str(&format!("{:02x}", color[0]));
    hex.push_str(&format!("{:02x}", color[1]));
    hex.push_str(&format!("{:02x}", color[2]));
    if color[3] != 255 {
        hex.push_str(&format!("{:02x}", color[3]));
    }

    hex
}

/// Allows to take String, string slices, arrays or slices of u8 (3 or 4) to create a [Color]
pub struct Color(pub String);

impl Color {
    /// Returns the contained color
    #[must_use]
    pub fn to_str(&self) -> &str {
        &self.0
    }
}

impl From<String> for Color {
    fn from(color: String) -> Self {
        Self(color)
    }
}

impl From<&str> for Color {
    fn from(color: &str) -> Self {
        Self(color.to_string())
    }
}

impl From<[u8; 4]> for Color {
    fn from(color: [u8; 4]) -> Self {
        Self(rgba2hex(color))
    }
}

impl From<[u8; 3]> for Color {
    fn from(color: [u8; 3]) -> Self {
        Self::from([color[0], color[1], color[2], 255])
    }
}

impl From<&[u8]> for Color {
    fn from(color: &[u8]) -> Self {
        if color.len() == 3 {
            Self::from([color[0], color[1], color[2]])
        } else if color.len() == 4 {
            Self::from([color[0], color[1], color[2], color[3]])
        } else {
            panic!("Invalid color length");
        }
    }
}

impl From<Vec<u8>> for Color {
    fn from(color: Vec<u8>) -> Self {
        Self::from(&color[..])
    }
}
