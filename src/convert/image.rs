//! Converts [`QRCode`] to an image
//!
//! ```rust
//! use fast_qr::convert::ConvertError;
//! use fast_qr::convert::{image::ImageBuilder, Builder, Shape};
//! use fast_qr::qr::QRBuilder;
//!
//! # fn main() -> Result<(), ConvertError> {
//! // QRBuilde::new can fail if content is too big for version,
//! // please check before unwrapping.
//! let qrcode = QRBuilder::new("https://example.com/")
//!     .build()
//!     .unwrap();
//!
//! let _img = ImageBuilder::default()
//!     .shape(Shape::RoundedSquare)
//!     .fit_width(600)
//!     .to_file(&qrcode, "out.png");
//!
//! #     std::fs::remove_file("out.png");
//! #     Ok(())
//! # }
//! ```

use std::fmt::Formatter;
use std::io;

use crate::QRCode;

use super::Color;
use super::{svg::SvgBuilder, Builder, ModuleShape};

use resvg::tiny_skia::{self, Pixmap};
use resvg::usvg;

/// [`ImageBuilder`] contains an [`SvgBuilder`] and adds some options \
/// - fit_height adds a max-height boundary
/// - fit_width adds a max-width boundary
pub struct ImageBuilder {
    fit_height: Option<u32>,
    fit_width: Option<u32>,
    svg_builder: SvgBuilder,
}

/// Error when converting to image
#[derive(Debug)]
pub enum ImageError {
    /// Error while writing to file
    IoError(io::Error),
    /// Error while creating image
    ImageError(String),
    /// Error while convert to bytes
    EncodingError(String),
}

impl std::error::Error for ImageError {}

impl std::fmt::Display for ImageError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ImageError::IoError(io_err) => f.write_str(io_err.to_string().as_str()),
            ImageError::ImageError(error) => f.write_str(error.as_str()),
            ImageError::EncodingError(error) => f.write_str(error.as_str()),
        }
    }
}

/// Creates an ImageBuilder instance, which contains an [`SvgBuilder`]
impl Default for ImageBuilder {
    fn default() -> Self {
        ImageBuilder {
            fit_height: None,
            fit_width: None,
            svg_builder: Default::default(),
        }
    }
}

impl Builder for ImageBuilder {
    fn margin(&mut self, margin: usize) -> &mut Self {
        self.svg_builder.margin(margin);
        self
    }

    fn module_color<C: Into<Color>>(&mut self, module_color: C) -> &mut Self {
        self.svg_builder.module_color(module_color);
        self
    }

    fn background_color<C: Into<Color>>(&mut self, background_color: C) -> &mut Self {
        self.svg_builder.background_color(background_color);
        self
    }

    fn module_shape(&mut self, shape: ModuleShape) -> &mut Self {
        self.svg_builder.module_shape(shape);
        self
    }

    fn image(&mut self, image: String) -> &mut Self {
        self.svg_builder.image(image);
        self
    }

    fn image_background_color<C: Into<Color>>(&mut self, image_background_color: C) -> &mut Self {
        self.svg_builder
            .image_background_color(image_background_color);
        self
    }

    fn image_background_shape(
        &mut self,
        image_background_shape: super::ImageBackgroundShape,
    ) -> &mut Self {
        self.svg_builder
            .image_background_shape(image_background_shape);
        self
    }

    fn image_size(&mut self, image_size: f64, gap: f64) -> &mut Self {
        self.svg_builder.image_size(image_size, gap);
        self
    }

    fn image_position(&mut self, x: f64, y: f64) -> &mut Self {
        self.svg_builder.image_position(x, y);
        self
    }

    fn module_shape_color<C: Into<Color>>(&mut self, shape: ModuleShape, color: C) -> &mut Self {
        self.svg_builder.module_shape_color(shape, color);
        self
    }

    fn eye_frame_shape(&mut self, shape: super::EyeFrameShape) -> &mut Self {
        self.svg_builder.eye_frame_shape(shape);
        self
    }

    fn eye_frame_shape_color<C: Into<Color>>(
        &mut self,
        shape: super::EyeFrameShape,
        color: C,
    ) -> &mut Self {
        self.svg_builder.eye_frame_shape_color(shape, color);
        self
    }
}

impl ImageBuilder {
    /// Add a max-height boundary
    pub fn fit_height(&mut self, height: u32) -> &mut Self {
        self.fit_height = Some(height);
        self
    }

    /// Add a max-width boundary
    pub fn fit_width(&mut self, width: u32) -> &mut Self {
        self.fit_width = Some(width);
        self
    }

    // From https://github.com/RazrFalcon/resvg/blob/374a25f/crates/resvg/tests/integration/main.rs
    /// Return a pixmap containing the svg for a QRCode
    pub fn to_pixmap(&self, qr: &QRCode) -> Pixmap {
        let opt = usvg::Options::default();

        // Do not unwrap on the from_data line, because panic will poison GLOBAL_OPT.
        let tree = {
            let svg_data = self.svg_builder.to_str(qr);
            let tree = usvg::Tree::from_data(svg_data.as_bytes(), &opt);
            tree.expect("Failed to parse SVG")
        };

        let fit_to = match (self.fit_width, self.fit_height) {
            (Some(w), Some(h)) => usvg::FitTo::Size(w, h),
            (Some(w), None) => usvg::FitTo::Width(w),
            (None, Some(h)) => usvg::FitTo::Height(h),
            _ => usvg::FitTo::Original,
        };

        let size = fit_to
            .fit_to(tree.size.to_screen_size())
            .unwrap_or(tree.size.to_screen_size());
        let mut pixmap =
            tiny_skia::Pixmap::new(size.width(), size.height()).expect("Failed to create pixmap");
        resvg::render(
            &tree,
            fit_to,
            tiny_skia::Transform::default(),
            pixmap.as_mut(),
        )
        .unwrap();

        pixmap
    }

    /// Saves the image for a QRCode to a file
    pub fn to_file(&self, qr: &QRCode, file: &str) -> Result<(), ImageError> {
        use io::{Error, ErrorKind};

        self.to_pixmap(qr)
            .save_png(file)
            .map_err(|err| ImageError::IoError(Error::new(ErrorKind::Other, err.to_string())))
    }

    /// Saves the image for a QRCode in a byte buffer
    pub fn to_bytes(&self, qr: &QRCode) -> Result<Vec<u8>, ImageError> {
        let out = self.to_pixmap(qr);
        out.encode_png()
            .map_err(|err| ImageError::EncodingError(err.to_string()))
    }
}
