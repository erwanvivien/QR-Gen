//! Converts a [`crate::QRCode`] to image or SVG you will need to activate associated feature flag

#[cfg(feature = "svg")]
#[cfg_attr(docsrs, doc(cfg(feature = "svg")))]
pub mod svg;

#[cfg(feature = "svg")]
use svg::SvgError;

#[cfg(feature = "image")]
#[cfg_attr(docsrs, doc(cfg(feature = "image")))]
pub mod image;
#[cfg(feature = "image")]
use image::ImageError;

pub mod color;
pub use color::{rgba2hex, Color};

mod module_shape;
pub use module_shape::{ModuleFunction, ModuleShape};

mod eye_shape;
pub use eye_shape::{EyeFrameShape, EyeFunction, EyePosition};

#[cfg(all(target_arch = "wasm32", feature = "wasm-bindgen"))]
use wasm_bindgen::prelude::*;

/// Different possible image background shapes
#[cfg_attr(feature = "wasm-bindgen", repr(C), wasm_bindgen)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub enum ImageBackgroundShape {
    /// Square shape
    Square,
    /// Circle shape
    Circle,
    /// Rounded square shape
    RoundedSquare,
}

/// Contains possible errors for a conversion
#[derive(Debug)]
pub enum ConvertError {
    /// Contains error message for a SVG conversion
    #[cfg(feature = "svg")]
    #[cfg_attr(docsrs, doc(cfg(feature = "svg")))]
    Svg(String),
    /// Contains error message for an Image conversion
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
            #[cfg(not(feature = "wasm-bindgen"))]
            SvgError::IoError(io_err) => Self::Io(io_err),
        }
    }
}

#[cfg(feature = "image")]
#[cfg_attr(docsrs, doc(cfg(feature = "image")))]
impl From<ImageError> for ConvertError {
    fn from(err: ImageError) -> Self {
        match err {
            ImageError::EncodingError(image_err) => Self::Image(image_err),
            ImageError::ImageError(image_err) => Self::Image(image_err),
            ImageError::IoError(io_err) => Self::Io(io_err),
        }
    }
}

/// Trait for `SvgBuilder` and `ImageBuilder`
pub trait Builder {
    /// Updates margin (default: 4)
    fn margin(&mut self, margin: usize) -> &mut Self;
    /// Updates module color (default: #000000)
    fn module_color<C: Into<Color>>(&mut self, module_color: C) -> &mut Self;
    /// Updates background color (default: #FFFFFF)
    fn background_color<C: Into<Color>>(&mut self, background_color: C) -> &mut Self;
    /// Adds a shape to the shapes list
    fn module_shape(&mut self, shape: ModuleShape) -> &mut Self;
    /// Add a shape to the shapes list with a specific color
    fn module_shape_color<C: Into<Color>>(&mut self, shape: ModuleShape, color: C) -> &mut Self;

    // Manages the eye part
    /// Adds a shape to the eye shapes list
    fn eye_frame_shape(&mut self, shape: EyeFrameShape) -> &mut Self;
    /// Add a shape to the eye shapes list with a specific color
    fn eye_frame_shape_color<C: Into<Color>>(
        &mut self,
        shape: EyeFrameShape,
        color: C,
    ) -> &mut Self;

    // Manages the image part

    /// Provides the image path or an base64 encoded image
    fn image(&mut self, image: String) -> &mut Self;
    /// Updates the image background color (default: #FFFFFF)
    fn image_background_color<C: Into<Color>>(&mut self, image_background_color: C) -> &mut Self;
    /// Updates the image background shape (default: Square)
    fn image_background_shape(&mut self, image_background_shape: ImageBackgroundShape)
        -> &mut Self;
    /// Updates the image size and the gap between the image and the [`crate::QRCode`]
    /// Default is around 30% of the [`crate::QRCode`] size
    fn image_size(&mut self, image_size: f64, gap: f64) -> &mut Self;
    /// Updates the image position, anchor is the center of the image. Default is the center of the [`crate::QRCode`]
    fn image_position(&mut self, x: f64, y: f64) -> &mut Self;
}
