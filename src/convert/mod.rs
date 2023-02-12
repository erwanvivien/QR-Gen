//! Converts a [`crate::QRCode`] to image or SVG you will need to activate associated feature flag

#[cfg(feature = "svg")]
#[cfg_attr(docsrs, doc(cfg(feature = "svg")))]
pub mod svg;
use core::ops::Deref;

#[cfg(feature = "svg")]
use svg::SvgError;

#[cfg(feature = "image")]
#[cfg_attr(docsrs, doc(cfg(feature = "image")))]
pub mod image;
#[cfg(feature = "image")]
use image::ImageError;

use crate::Module;

/// Converts a position to a module svg
/// # Example
///
/// For the square shape, the svg is `M{x},{y}h1v1h-1`
///
/// ```rust
/// fn square(y: usize, x: usize) -> String {
///     format!("M{},{}h1v1h-1", x, y)
/// }
/// ```
pub type ModuleFunction = fn(usize, usize, Module) -> String;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

/// Different possible Shapes to represent modules in a QRCode
#[cfg_attr(target_arch = "wasm32", repr(C), wasm_bindgen)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub enum Shape {
    /// Square Shape
    Square,
    /// Circle Shape
    Circle,
    /// RoundedSquare Shape
    RoundedSquare,
    /// Vertical Shape
    Vertical,
    /// Horizontal Shape
    Horizontal,
    /// Diamond Shape
    Diamond,
}

impl From<Shape> for usize {
    fn from(shape: Shape) -> Self {
        match shape {
            Shape::Square => 0,
            Shape::Circle => 1,
            Shape::RoundedSquare => 2,
            Shape::Vertical => 3,
            Shape::Horizontal => 4,
            Shape::Diamond => 5,
            #[cfg(not(target_arch = "wasm32"))]
            Shape::Command(_) => 6,
        }
    }
}

impl From<String> for Shape {
    fn from(shape: String) -> Self {
        match shape.to_lowercase().as_str() {
            "square" => Shape::Square,
            "circle" => Shape::Circle,
            "rounded_square" => Shape::RoundedSquare,
            "vertical" => Shape::Vertical,
            "horizontal" => Shape::Horizontal,
            "diamond" => Shape::Diamond,

            _ => Shape::Square,
        }
    }
}

impl From<Shape> for &str {
    fn from(shape: Shape) -> Self {
        match shape {
            Shape::Square => "square",
            Shape::Circle => "circle",
            Shape::RoundedSquare => "rounded_square",
            Shape::Vertical => "vertical",
            Shape::Horizontal => "horizontal",
            Shape::Diamond => "diamond",
            #[cfg(not(target_arch = "wasm32"))]
            Shape::Command(_) => "command",
        }
    }
}

impl Shape {
    pub(crate) fn square(y: usize, x: usize, _: Module) -> String {
        format!("M{},{}h1v1h-1", x, y)
    }

    pub(crate) fn circle(y: usize, x: usize, _: Module) -> String {
        format!("M{},{}a.5,.5 0 1,1 0,-.1", x + 1, y as f32 + 0.5f32)
    }

    pub(crate) fn rounded_square(y: usize, x: usize, _: Module) -> String {
        format!("M{0}.2,{1}.2 {0}.8,{1}.2 {0}.8,{1}.8 {0}.2,{1}.8z", x, y)
    }

    pub(crate) fn horizontal(y: usize, x: usize, _: Module) -> String {
        format!("M{},{}.1h1v.8h-1", x, y)
    }

    pub(crate) fn vertical(y: usize, x: usize, _: Module) -> String {
        format!("M{}.1,{}h.8v1h-.8", x, y)
    }

    pub(crate) fn diamond(y: usize, x: usize, _: Module) -> String {
        format!("M{}.5,{}l.5,.5l-.5,.5l-.5,-.5z", x, y)
    }

    const FUNCTIONS: [ModuleFunction; 6] = [
        Shape::square,
        Shape::circle,
        Shape::rounded_square,
        Shape::vertical,
        Shape::horizontal,
        Shape::diamond,
    ];
}

impl Deref for Shape {
    type Target = ModuleFunction;

    fn deref(&self) -> &Self::Target {
        let index: usize = (*self).into();
        match self {
            #[cfg(not(target_arch = "wasm32"))]
            Self::Command(func) => func,
            _ => &Self::FUNCTIONS[index],
        }
    }
}

/// Different possible image background shapes
#[cfg_attr(target_arch = "wasm32", repr(C), wasm_bindgen)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub enum ImageBackgroundShape {
    /// Square shape
    Square,
    /// Circle shape
    Circle,
    /// Rounded square shape
    RoundedSquare,
}

/// Contains possible errors for a convertion
#[derive(Debug)]
pub enum ConvertError {
    /// Contains error message for a SVG convertion
    #[cfg(feature = "svg")]
    #[cfg_attr(docsrs, doc(cfg(feature = "svg")))]
    Svg(String),
    /// Contains error message for an Image convertion
    #[cfg(feature = "image")]
    #[cfg_attr(docsrs, doc(cfg(feature = "image")))]
    Image(String),
    /// Contains error message if a file write failed
    Io(std::io::Error),
}

#[cfg(feature = "svg")]
#[cfg_attr(docsrs, doc(cfg(feature = "svg")))]
impl From<SvgError> for ConvertError {
    fn from(err: SvgError) -> Self {
        match err {
            SvgError::SvgError(svg_err) => Self::Svg(svg_err),
            #[cfg(not(target_arch = "wasm32"))]
            SvgError::IoError(io_err) => Self::Io(io_err),
        }
    }
}

#[cfg(feature = "image")]
#[cfg_attr(docsrs, doc(cfg(feature = "image")))]
impl From<ImageError> for ConvertError {
    fn from(err: ImageError) -> Self {
        match err {
            ImageError::ImageError(image_err) => Self::Image(image_err),
            ImageError::IoError(io_err) => Self::Io(io_err),
        }
    }
}

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

/// Trait for `SvgBuilder` and `ImageBuilder`
pub trait Builder {
    /// Updates margin (default: 4)
    fn margin(&mut self, margin: usize) -> &mut Self;
    /// Updates module color (default: #000000)
    fn module_color(&mut self, module_color: [u8; 4]) -> &mut Self;
    /// Updates background color (default: #FFFFFF)
    fn background_color(&mut self, background_color: [u8; 4]) -> &mut Self;
    /// Adds a shape to the shapes list
    fn shape(&mut self, shape: Shape) -> &mut Self;
    /// Add a shape to the shapes list with a specific color
    fn shape_color(&mut self, shape: Shape, color: [u8; 4]) -> &mut Self;

    // Manages the image part

    /// Provides the image path or an base64 encoded image
    fn image(&mut self, image: String) -> &mut Self;
    /// Updates the image background color (default: #FFFFFF)
    fn image_background_color(&mut self, image_background_color: [u8; 4]) -> &mut Self;
    /// Updates the image background shape (default: Square)
    fn image_background_shape(&mut self, image_background_shape: ImageBackgroundShape)
        -> &mut Self;
    /// Updates the image size and the gap between the image and the QRCode
    /// Default is around 30% of the QRCode size
    fn image_size(&mut self, image_size: f64, gap: f64) -> &mut Self;
    /// Updates the image position, anchor is the center of the image. Default is the center of the QRCode
    fn image_position(&mut self, x: f64, y: f64) -> &mut Self;
}
