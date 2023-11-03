//! Converts [`QRCode`] to SVG
//!
//! ```rust
//! use fast_qr::convert::ConvertError;
//! use fast_qr::convert::{svg::SvgBuilder, Builder, Shape};
//! use fast_qr::qr::QRBuilder;
//!
//! # fn main() -> Result<(), ConvertError> {
//! // QRBuilde::new can fail if content is too big for version,
//! // please check before unwrapping.
//! let qrcode = QRBuilder::new("https://example.com/")
//!     .build()
//!     .unwrap();
//!
//! let _svg = SvgBuilder::default()
//!     .shape(Shape::RoundedSquare)
//!     .to_file(&qrcode, "out.svg");
//!
//! #     std::fs::remove_file("out.svg");
//! #     Ok(())
//! # }
//! ```

use crate::{convert::EyePosition, ModuleType, QRCode, Version};

use super::{
    module_shape::ModuleFunction, Builder, Color, EyeFrameShape, ImageBackgroundShape, ModuleShape,
};

/// Builder for svg, can set shape, margin, background_color, dot_color
pub struct SvgBuilder {
    /// Command vector allows predefined or custom shapes
    /// The default is square, commands can be added using `.shape()`
    commands: Vec<ModuleFunction>,
    /// Commands can also have a custom color
    /// The default is `dot_color`, commands with specific colors can be
    /// added using `.shape_color()`
    command_colors: Vec<Option<Color>>,
    /// The margin for the svg, default is 4
    margin: usize,
    /// The background color for the svg, default is #FFFFFF
    background_color: Color,
    /// The color for each module, default is #000000
    dot_color: Color,

    // Eye Frame
    /// Eye Frame Shape
    eye_frame_shape: EyeFrameShape,
    /// Eye Frame Color
    eye_frame_color: Color,

    // Image Embedding
    /// Image to embed in the svg, can be a path or a base64 string
    image: Option<String>,
    /// Background color for the image, default is #FFFFFF
    image_background_color: Color,
    /// Background shape for the image, default is square
    image_background_shape: ImageBackgroundShape,
    /// Size of the image, default is ~1/3 of the svg
    image_size: Option<(f64, f64)>,
    /// Position of the image, default is center
    image_position: Option<(f64, f64)>,
}

#[derive(Debug)]
/// Possible errors when converting to SVG
pub enum SvgError {
    /// Error while writing file
    #[cfg(not(feature = "wasm-bindgen"))]
    IoError(std::io::Error),
    /// Error while creating svg
    SvgError(String),
}

/// Creates a Builder instance
impl Default for SvgBuilder {
    fn default() -> Self {
        SvgBuilder {
            background_color: [255; 4].into(),
            dot_color: [0, 0, 0, 255].into(),
            margin: 4,
            commands: Vec::new(),
            command_colors: Vec::new(),

            eye_frame_shape: EyeFrameShape::Empty,
            eye_frame_color: [0, 0, 0, 255].into(),

            // Image Embedding
            image: None,
            image_background_color: [255; 4].into(),
            image_background_shape: ImageBackgroundShape::Square,
            image_size: None,
            image_position: None,
        }
    }
}

impl Builder for SvgBuilder {
    fn margin(&mut self, margin: usize) -> &mut Self {
        self.margin = margin;
        self
    }

    fn module_color<C: Into<Color>>(&mut self, dot_color: C) -> &mut Self {
        self.dot_color = dot_color.into();
        self
    }

    fn background_color<C: Into<Color>>(&mut self, background_color: C) -> &mut Self {
        self.background_color = background_color.into();
        self
    }

    fn module_shape(&mut self, shape: ModuleShape) -> &mut Self {
        self.commands.push(*shape);
        self.command_colors.push(None);
        self
    }

    fn module_shape_color<C: Into<Color>>(&mut self, shape: ModuleShape, color: C) -> &mut Self {
        self.commands.push(*shape);
        self.command_colors.push(Some(color.into()));
        self
    }

    fn image(&mut self, image: String) -> &mut Self {
        self.image = Some(image);
        self
    }

    fn image_background_color<C: Into<Color>>(&mut self, image_background_color: C) -> &mut Self {
        self.image_background_color = image_background_color.into();
        self
    }

    fn image_background_shape(
        &mut self,
        image_background_shape: ImageBackgroundShape,
    ) -> &mut Self {
        self.image_background_shape = image_background_shape;
        self
    }

    fn image_size(&mut self, image_size: f64, gap: f64) -> &mut Self {
        self.image_size = Some((image_size, gap));
        self
    }

    fn image_position(&mut self, x: f64, y: f64) -> &mut Self {
        self.image_position = Some((x, y));
        self
    }

    fn eye_frame_shape(&mut self, shape: EyeFrameShape) -> &mut Self {
        self.eye_frame_shape = shape;
        self
    }

    fn eye_frame_shape_color<C: Into<Color>>(
        &mut self,
        shape: EyeFrameShape,
        color: C,
    ) -> &mut Self {
        self.eye_frame_shape = shape;
        self.eye_frame_color = color.into();
        self
    }
}

impl SvgBuilder {
    /// Return the coordinates of the eye according to the eye position
    ///
    /// Return (x, y)
    fn eye_placement(&self, qr: &QRCode, eye_position: EyePosition) -> (usize, usize) {
        let margin = self.margin;
        let offset = qr.size + margin - 7;

        match eye_position {
            EyePosition::TopLeft => (margin, margin),
            EyePosition::TopRight => (offset, margin),
            EyePosition::BottomLeft => (margin, offset),
        }
    }

    fn image_placement(
        image_background_shape: ImageBackgroundShape,
        margin: usize,
        n: usize,
    ) -> (f64, (f64, f64), f64) {
        use ImageBackgroundShape::{Circle, RoundedSquare, Square};

        // (border_size, placed_coord)
        #[rustfmt::skip]
        const SQUARE: [(f64, f64); 40] = [
            (5f64, 8f64),   (9f64, 8f64),   (9f64, 10f64),  (11f64, 11f64), (13f64, 12f64),
            (13f64, 14f64), (15f64, 15f64), (17f64, 16f64), (17f64, 18f64), (19f64, 19f64),
            (21f64, 20f64), (21f64, 22f64), (23f64, 23f64), (25f64, 24f64), (25f64, 26f64),
            (27f64, 27f64), (29f64, 28f64), (29f64, 30f64), (31f64, 31f64), (33f64, 32f64),
            (33f64, 34f64), (35f64, 35f64), (37f64, 36f64), (37f64, 38f64), (39f64, 39f64),
            (41f64, 40f64), (41f64, 42f64), (43f64, 43f64), (45f64, 44f64), (45f64, 46f64),
            (47f64, 47f64), (49f64, 48f64), (49f64, 50f64), (51f64, 51f64), (53f64, 52f64),
            (53f64, 54f64), (55f64, 55f64), (57f64, 56f64), (57f64, 58f64), (59f64, 59f64),
        ];
        const ROUNDED_SQUARE: [(f64, f64); 40] = SQUARE;
        const CIRCLE: [(f64, f64); 40] = SQUARE;

        // Using hardcoded values
        let version = Version::from_n(n) as usize;
        let (border_size, placed_coord) = match image_background_shape {
            Square => SQUARE[version],
            RoundedSquare => ROUNDED_SQUARE[version],
            Circle => CIRCLE[version],
        };

        // Allows for a module gap between the image and the border
        let gap = match image_background_shape {
            Square | RoundedSquare => 2f64,
            Circle => 3f64,
        };
        // Make the image border bigger for bigger versions
        let gap = gap * (version + 10) as f64 / 10f64;
        let placed_coord = placed_coord + margin as f64;
        let placed_coord = (placed_coord, placed_coord);

        (border_size, placed_coord, border_size - gap)
    }

    fn image(&self, n: usize) -> String {
        if self.image.is_none() {
            return String::new();
        }

        let image = self.image.as_ref().unwrap();
        let mut out = String::with_capacity(image.len() + 100);

        let (mut border_size, mut placed_coord, mut image_size) =
            Self::image_placement(self.image_background_shape, self.margin, n);

        if let Some((override_size, gap)) = self.image_size {
            border_size = override_size + gap * 2f64;
            let mut placed_coord_x = (self.margin * 2 + n) as f64 - border_size;
            placed_coord_x /= 2f64;
            placed_coord = (placed_coord_x, placed_coord_x);
            image_size = override_size;
        }

        if let Some((x, y)) = self.image_position {
            placed_coord = (x - border_size / 2f64, y - border_size / 2f64);
        }

        out.push_str(&format!(
            r#"<rect x="{0:.2}" y="{1:.2}" width="{2:.2}" height="{2:.2}" fill="{3}"/>"#,
            placed_coord.0,
            placed_coord.1,
            border_size,
            self.background_color.to_str()
        ));

        let format = match self.image_background_shape {
            ImageBackgroundShape::Square => {
                r#"<rect x="{0}" y="{1}" width="{2}" height="{2}" fill="{3}"/>"#
            }
            ImageBackgroundShape::Circle => {
                r#"<rect x="{0}" y="{1}" width="{2}" height="{2}" fill="{3}" rx="1000px"/>"#
            }
            ImageBackgroundShape::RoundedSquare => {
                r#"<rect x="{0}" y="{1}" width="{2}" height="{2}" fill="{3}" rx="1px"/>"#
            }
        };

        let format = format
            .replace("{0}", &placed_coord.0.to_string())
            .replace("{1}", &placed_coord.1.to_string())
            .replace("{2}", &border_size.to_string())
            .replace("{3}", &self.image_background_color.to_str());

        out.push_str(&format);

        out.push_str(&format!(
            r#"<image x="{0:.2}" y="{1:.2}" width="{2:.2}" height="{2:.2}" href="{3}" />"#,
            placed_coord.0 + (border_size - image_size) / 2f64,
            placed_coord.1 + (border_size - image_size) / 2f64,
            image_size,
            image
        ));

        out
    }

    fn path(&self, qr: &QRCode) -> String {
        const DEFAULT_COMMAND: [ModuleFunction; 1] = [ModuleShape::square];
        const DEFAULT_COMMAND_COLOR: [Option<Color>; 1] = [None];

        // TODO: cleanup this basic logic
        let command_colors: &[Option<Color>] = if !self.commands.is_empty() {
            &self.command_colors
        } else {
            &DEFAULT_COMMAND_COLOR
        };
        let commands: &[ModuleFunction] = if !self.commands.is_empty() {
            &self.commands
        } else {
            &DEFAULT_COMMAND
        };

        let mut paths = vec![String::with_capacity(10 * qr.size * qr.size); commands.len()];
        for path in paths.iter_mut() {
            path.push_str(r#"<path d=""#);
        }

        for y in 0..qr.size {
            let line = &qr[y];
            for (x, &cell) in line.iter().enumerate() {
                if !cell.value() {
                    continue;
                }

                if self.eye_frame_shape != EyeFrameShape::Empty
                    && cell.module_type() == ModuleType::FinderPattern
                {
                    continue;
                }

                for (i, command) in commands.iter().enumerate() {
                    paths[i].push_str(&command(x + self.margin, y + self.margin, cell));
                }
            }
        }

        for (i, &command) in commands.iter().enumerate() {
            let command_color = command_colors[i].as_ref().unwrap_or(&self.dot_color);
            // Allows to compare if two function pointers are the same
            // This works because there is no notion of Generics for `rounded_square`
            if command as usize == ModuleShape::rounded_square as usize {
                paths[i].push_str(&format!(
                    r##"" stroke-width=".3" stroke-linejoin="round" stroke="{}"##,
                    command_color.to_str()
                ));
            }

            paths[i].push_str(&format!(r#"" fill="{}"/>"#, command_color.to_str()));
        }

        let mut finder_pattern_path = String::with_capacity(128);
        if self.eye_frame_shape != EyeFrameShape::Empty {
            for eye_position in EyePosition::ALL {
                let (x, y) = self.eye_placement(qr, eye_position);

                let eye_frame_shape_function = self.eye_frame_shape;
                let eye_shape_str = eye_frame_shape_function(y, x, eye_position);

                finder_pattern_path.push_str(&eye_shape_str);
            }

            paths.push(finder_pattern_path);
        }

        paths.join("")
    }

    /// Return a string containing the svg for a qr code
    pub fn to_str(&self, qr: &QRCode) -> String {
        let n = qr.size;

        let mut out = String::with_capacity(11 * n * n / 2);
        out.push_str(&format!(
            r#"<svg viewBox="0 0 {0} {0}" xmlns="http://www.w3.org/2000/svg">"#,
            self.margin * 2 + n
        ));

        out.push_str(&format!(
            r#"<rect width="{0}px" height="{0}px" fill="{1}"/>"#,
            self.margin * 2 + n,
            self.background_color.to_str()
        ));

        out.push_str(&self.path(qr));
        out.push_str(&self.image(n));

        out.push_str("</svg>");
        out
    }

    /// Saves the svg for a qr code to a file
    #[cfg(not(feature = "wasm-bindgen"))]
    pub fn to_file(&self, qr: &QRCode, file: &str) -> Result<(), SvgError> {
        use std::fs::File;
        use std::io::Write;

        let out = self.to_str(qr);

        let mut f = File::create(file).map_err(SvgError::IoError)?;
        f.write_all(out.as_bytes()).map_err(SvgError::IoError)?;

        Ok(())
    }
}
