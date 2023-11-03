/// Corresponds to the position of the Eye
/// - TopLeft
/// - TopRight
/// - BottomRight
#[derive(Debug, Clone, Copy)]
pub enum EyePosition {
    /// Top left eye
    TopLeft,
    /// Top right eye
    TopRight,
    /// Bottom left eye
    BottomLeft,
}

impl EyePosition {
    /// Iterates over all possible eye positions
    ///
    /// TopLeft, TopRight, BottomLeft
    pub const ALL: [Self; 3] = [Self::TopLeft, Self::TopRight, Self::BottomLeft];
}

/// Converts an eye position to a custom svg
///
/// # Example
///
/// For the fully squared shape, the svg is `M{x},{y}h7v7h-7`
///
/// The eye function for the eye frame should be max of 7x7. \
/// The eye function for the eye ball should be max of 3x3.
///
/// ```rust
/// fn square(y: usize, x: usize, _: EyePosition) -> String {
///     format!("h7v7h-7")
/// }
/// ```
pub type EyeFunction = fn(usize, usize, EyePosition) -> String;

// TODO: Find a way to use the same enum for wasm and not wasm
// Current bug being that wasm_bindgen & #[cfg(not(target_arch = "wasm32"))] are not compatible(?)
/// Different possible Shapes to represent modules in a [`crate::QRCode`]
#[repr(C)]
#[wasm_bindgen]
#[cfg(feature = "wasm-bindgen")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub enum EyeFrameShape {
    /// Square shape
    Square,
    /// Rounded square shape
    Rounded,
    /// Circle shape
    Circle,
    /// Rounded square shape with the outer corner rounded
    RoundedSquaredOuterCorner,
    /// Leaf shape
    Leaf,
    /// Rounded square shape with all but the inner corner rounded
    RoundedSquaredInnerCorner,
    /// Square shape with a dot in the middle
    DottedSquare,
    /// Eye lash shape
    EyeLash,
}

/// Different possible Shapes to represent modules in a [`crate::QRCode`]
#[cfg(not(feature = "wasm-bindgen"))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub enum EyeFrameShape {
    /// Empty shape, most often used with [`ModuleShape::Custom`]
    Empty,
    /// Square shape
    Square,
    /// Rounded square shape
    Rounded,
    /// Circle shape
    Circle,
    /// Rounded square shape with the outer corner rounded
    RoundedSquaredOuterCorner,
    /// Leaf shape
    Leaf,
    /// Rounded square shape with all but the inner corner rounded
    RoundedSquaredInnerCorner,
    /// Eye lash shape
    EyeLash,
    /// Custom Shape with a function / closure
    /// # Example
    /// ```rust
    /// use fast_qr::convert::EyeFrameShape;
    /// let command_function = |eye_position| {
    ///     match eye_position {
    ///         EyePosition::TopLeft => String::from("..."),
    ///         _ => String::from("..."),
    ///     }
    /// };
    /// let command = EyeFrameShape::Command(command_function);
    /// ```
    Command(EyeFunction),
}

impl From<EyeFrameShape> for usize {
    fn from(shape: EyeFrameShape) -> Self {
        match shape {
            EyeFrameShape::Empty => 0,
            EyeFrameShape::Square => 1,
            EyeFrameShape::Rounded => 2,
            EyeFrameShape::Circle => 3,
            EyeFrameShape::RoundedSquaredOuterCorner => 4,
            EyeFrameShape::Leaf => 5,
            EyeFrameShape::RoundedSquaredInnerCorner => 6,
            EyeFrameShape::EyeLash => 7,
            #[cfg(not(feature = "wasm-bindgen"))]
            EyeFrameShape::Command(_) => 8,
        }
    }
}

impl From<String> for EyeFrameShape {
    #[allow(clippy::match_same_arms)]
    fn from(shape: String) -> Self {
        match shape.as_ref() {
            "empty" => Self::Empty,
            "square" => Self::Square,
            "rounded" => Self::Rounded,
            "circle" => Self::Circle,
            "rounded_squared_outer_corner" => Self::RoundedSquaredOuterCorner,
            "leaf" => Self::Leaf,
            "rounded_squared_side_3" => Self::RoundedSquaredInnerCorner,
            "eye_lash" => Self::EyeLash,

            _ => Self::Square,
        }
    }
}

impl From<EyeFrameShape> for &str {
    fn from(shape: EyeFrameShape) -> Self {
        match shape {
            EyeFrameShape::Empty => "empty",
            EyeFrameShape::Square => "square",
            EyeFrameShape::Rounded => "rounded",
            EyeFrameShape::Circle => "circle",
            EyeFrameShape::RoundedSquaredOuterCorner => "rounded_squared_outer_corner",
            EyeFrameShape::Leaf => "leaf",
            EyeFrameShape::RoundedSquaredInnerCorner => "rounded_squared_side_3",
            EyeFrameShape::EyeLash => "eye_lash",

            #[cfg(not(feature = "wasm-bindgen"))]
            EyeFrameShape::Command(_) => "command",
        }
    }
}

impl EyeFrameShape {
    fn inner_black_rect(y: usize, x: usize) -> String {
        let x_rect_offset = x + 2;
        let y_rect_offset = y + 2;

        let black_rect_path = format!("M{x_rect_offset},{y_rect_offset}h3v3h-3z");

        format!(r#"<path d="{black_rect_path}" fill="currentColor"/>"#)
    }

    pub(crate) const fn empty(_: usize, _: usize, _: EyePosition) -> String {
        String::new()
    }

    pub(crate) fn square(y: usize, x: usize, _: EyePosition) -> String {
        let offset_x = x + 6;
        let offset_y = y + 6;

        let x_1 = x + 1;
        let y_1 = y + 1;

        let x_2 = x + 2;
        let y_2 = y + 2;

        let d_path = format!("M{x_1},{y}h6v1h-6zM{x},{y}v7h1v-7zM{offset_x},{y_1}v6h1v-6zM{x_1},{offset_y}h5v1h-5zM{x_2},{y_2}h3v3h-3z");
        format!(r#"<path d="{d_path}" fill="currentColor"/>"#)
    }

    pub(crate) fn rounded(y: usize, x: usize, _: EyePosition) -> String {
        const TRANSPARENT: &str = "#0000";

        let stroked_rect = format!(
            r#"<rect rx="2" fill="{TRANSPARENT}" stroke="black" width="6" height="6" y="{x}.5" x="{y}.5"/>"#
        );
        let black_rect_path = EyeFrameShape::inner_black_rect(y, x);

        format!(r"{stroked_rect}{black_rect_path}")
    }

    pub(crate) fn circle(y: usize, x: usize, _: EyePosition) -> String {
        let x_circle_center = x + 3;
        let y_circle_center = y + 3;

        let stroked_circle = format!(
            r#"<circle r="3.5" fill="none" stroke="black" cx="{x_circle_center}.5" cy="{y_circle_center}.5"/>"#
        );
        let black_rect_path = EyeFrameShape::inner_black_rect(y, x);

        format!(r"{stroked_circle}{black_rect_path}")
    }

    pub(crate) fn rounded_squared_outer_corner(
        y: usize,
        x: usize,
        eye_position: EyePosition,
    ) -> String {
        const WHITE: &str = "#fff";

        let x_inner = x + 1;
        let y_inner = y + 1;

        let outer_shape_path = match eye_position {
            EyePosition::TopLeft => format!(r#"M{x},{}v-5q0,-2 2,-2h5v7z"#, y + 7),
            EyePosition::TopRight => format!(r#"M{x},{y}h5q2,0 2,2v5h-7z"#),
            EyePosition::BottomLeft => format!(r#"M{x},{y}v5q0,2 2,2h5v-7z"#),
        };

        let inner_shape_path = match eye_position {
            EyePosition::TopLeft => format!(r#"M{x_inner},{}v-4q0,-1 1,-1h4v5z"#, y + 6),
            EyePosition::TopRight => format!(r#"M{x_inner},{y_inner}h4q1,0 1,1v4h-5z"#),
            EyePosition::BottomLeft => format!(r#"M{x_inner},{y_inner}v4q0,1 1,1h4v-5z"#),
        };

        let black_rect_path = EyeFrameShape::inner_black_rect(y, x);

        format!(
            r#"<path d="{outer_shape_path}" fill="currentColor"/><path d="{inner_shape_path}" fill="{WHITE}"/>{black_rect_path}"#,
        )
    }

    pub(crate) fn leaf(y: usize, x: usize, eye_position: EyePosition) -> String {
        const WHITE: &str = "#fff";

        let x_inner = x + 1;
        let y_inner = y + 1;

        let outer_shape_path = match eye_position {
            EyePosition::TopLeft => format!(r#"M{x},{}v-5q0,-2 2,-2h5v5q0,2 -2,2z"#, y + 7),
            EyePosition::TopRight | EyePosition::BottomLeft => {
                format!(r#"M{x},{y}h5q2,0 2,2v5h-5q-2,0 -2,-2z"#)
            }
        };

        let inner_shape_path = match eye_position {
            EyePosition::TopLeft => format!(r#"M{x_inner},{}v-4q0,-1 1,-1h4v4q0,1 -1,1z"#, y + 6),
            EyePosition::TopRight | EyePosition::BottomLeft => {
                format!(r#"M{x_inner},{y_inner}h4q1,0 1,1v4h-4q-1,0 -1,-1z"#)
            }
        };

        let black_rect_path = EyeFrameShape::inner_black_rect(y, x);

        format!(
            r#"<path d="{outer_shape_path}" fill="currentColor"/><path d="{inner_shape_path}" fill="{WHITE}"/>{black_rect_path}"#,
        )
    }

    pub(crate) fn rounded_squared_inner_corner(
        y: usize,
        x: usize,
        eye_position: EyePosition,
    ) -> String {
        const WHITE: &str = "#fff";

        let x_6 = x + 6;
        let y_6 = y + 6;
        let x_7 = x + 7;
        let y_7 = y + 7;

        let x_inner = x + 1;
        let y_inner = y + 1;

        let outer_shape_path = match eye_position {
            EyePosition::TopLeft => {
                format!(r#"M{x_7},{y_7}h-5q-2,0 -2,-2v-3q0,-2 2,-2h3q2,0 2,2"#)
            }
            EyePosition::TopRight => {
                format!(r#"M{x},{y_7}v-5q0,-2 2,-2h3q2,0 2,2v3q0,2 -2,2"#)
            }

            EyePosition::BottomLeft => {
                format!(r#"M{x_7},{y}v5q0,2 -2,2h-3q-2,0 -2,-2v-3q0,-2 2,-2"#)
            }
        };

        let inner_shape_path = match eye_position {
            EyePosition::TopLeft => {
                format!(r#"M{x_6},{y_6}h-4q-1,0 -1,-1v-3q0,-1 1,-1h3q1,0 1,1"#)
            }
            EyePosition::TopRight => {
                format!(r#"M{x_inner},{y_6}v-4q0,-1 1,-1h3q1,0 1,1v3q0,1 -1,1"#)
            }
            EyePosition::BottomLeft => {
                format!(r#"M{x_6},{y_inner}v4q0,1 -1,1h-3q-1,0 -1,-1v-3q0,-1 1,-1"#)
            }
        };

        let black_rect_path = EyeFrameShape::inner_black_rect(y, x);

        format!(
            r#"<path d="{outer_shape_path}" fill="currentColor"/><path d="{inner_shape_path}" fill="{WHITE}"/>{black_rect_path}"#,
        )
    }

    pub(crate) fn eye_lash(y: usize, x: usize, eye_position: EyePosition) -> String {
        const WHITE: &str = "#fff";

        let x_inner = x + 1;
        let y_inner = y + 1;

        let x_1 = x - 1;
        let y_1 = y - 1;

        let outer_shape_path = match eye_position {
            EyePosition::TopLeft => format!(r#"M{x_1}.5,{y_1}.5l5.5 .5q2,0 2,2v5h-5q-2,0 -2,-2z"#),
            EyePosition::TopRight => {
                format!(r#"M{x},{}v-5q0,-2 2,-2l5.5 -.5l-.5 5.5q0,2 -2,2z"#, y + 7)
            }
            EyePosition::BottomLeft => {
                format!(r#"M{x_1}.5,{}.5l.5 -5.5q0,-2 2,-2h5v5q0,2 -2,2z"#, y + 7)
            }
        };

        let inner_shape_path = match eye_position {
            EyePosition::TopLeft => format!(r#"M{x_inner},{y_inner}h4q1,0 1,1v4h-4q-1,0 -1,-1z"#),
            EyePosition::TopRight => format!(r#"M{x_inner},{}v-4q0,-1 1,-1h4v4q0,1 -1,1z"#, y + 6),
            EyePosition::BottomLeft => {
                format!(r#"M{x_inner},{}v-4q0,-1 1,-1h4v4q0,1 -1,1z"#, y + 6)
            }
        };

        let black_rect_path = EyeFrameShape::inner_black_rect(y, x);

        format!(
            r#"<path d="{outer_shape_path}" fill="currentColor"/><path d="{inner_shape_path}" fill="{WHITE}"/>{black_rect_path}"#,
        )
    }

    const FUNCTIONS: [EyeFunction; 8] = [
        EyeFrameShape::empty,
        EyeFrameShape::square,
        EyeFrameShape::rounded,
        EyeFrameShape::circle,
        EyeFrameShape::rounded_squared_outer_corner,
        EyeFrameShape::leaf,
        EyeFrameShape::rounded_squared_inner_corner,
        EyeFrameShape::eye_lash,
    ];
}

impl core::ops::Deref for EyeFrameShape {
    type Target = EyeFunction;

    fn deref(&self) -> &Self::Target {
        let index: usize = (*self).into();
        match self {
            #[cfg(not(target_arch = "wasm32"))]
            Self::Command(func) => func,
            _ => &Self::FUNCTIONS[index],
        }
    }
}
