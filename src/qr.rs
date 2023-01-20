//! Module `qr` is the entrypoint to start making `QRCodes`

use crate::module::Module;
use std::fmt::{Debug, Formatter};
use std::ops::{Index, IndexMut};

use crate::datamasking::Mask;
use crate::encode::Mode;
#[cfg(not(target_arch = "wasm32"))]
use crate::helpers;
use crate::{encode, Version, ECL};

/// A `QRCode` can be created using [`QRBuilder`]. Simple API for simple usage.
/// If you need to use `QRCode` directly, please file an [issue on
/// github](https://github.com/erwanvivien/fast_qr) explaining your use case.
///
/// Contains all needed information about the `QRCode`.
/// This is the main struct of the crate.
///
/// It contains the matrix of the `QRCode`, stored as a one-dimensional array.
#[derive(Clone)]
pub struct QRCode {
    /// This array length is of size `177 x 177`. It is using a fixed size
    /// array simply because of perfomance.
    ///
    /// # Other data type possible:
    /// - Templated Matrix was faster but crate size was huge.
    /// - Vector using `with_capacity`, really bad.
    pub data: [Module; 177 * 177],
    /// Width & Height of QRCode. If manually set, should be `version * 4 + 17`, `version` going
    /// from 1 to 40 both included.
    pub size: usize,

    /// Version of the `QRCode`, impacts the size.
    ///
    /// `None` will optimize Version according to ECL and Mode
    pub version: Option<Version>,
    /// Defines how powerfull `QRCode` redundancy should be or how much percent of a QRCode can be
    /// recovered.
    ///
    /// - `ECL::L`: 7%
    /// - `ECL::M`: 15%
    /// - `ECL::Q`: 25%
    /// - `ELC::H`: 30%
    ///
    /// `None` will set ECL to Quartile (`ELC::Q`)
    pub ecl: Option<ECL>,

    /// Changes the final pattern used.
    ///
    /// None will find the best suited mask.
    pub mask: Option<Mask>,
    /// Mode defines which data is being parsed, between Numeric, AlphaNumeric & Byte.
    ///
    /// `None` will optimize Mode according to user input.
    ///
    /// ## Note
    /// Kanji mode is not supported (yet).
    pub mode: Option<Mode>,
}

impl QRCode {
    /// A default `QRCode` will have all it's fields as `None` and a default Matrix filled with `Module::LIGHT`.
    #[must_use]
    pub const fn default(size: usize) -> Self {
        QRCode {
            data: [Module::data(Module::LIGHT); 177 * 177],
            size,
            version: None,
            ecl: None,
            mask: None,
            mode: None,
        }
    }
}

impl Index<usize> for QRCode {
    type Output = [Module];

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index * self.size..(index + 1) * self.size]
    }
}

impl IndexMut<usize> for QRCode {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index * self.size..(index + 1) * self.size]
    }
}

/// Contains different error when [`QRCode`] could not be created
pub enum QRCodeError {
    /// If data if too large to be encoded (refer to Table 7-11 of the spec or [an online table](https://fast-qr.com/blog/tables/ecl))
    EncodedData,
    /// Specified version too small to contain data
    SpecifiedVersion,
}

impl Debug for QRCodeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            QRCodeError::EncodedData => f.write_str("Data too big to be encoded"),
            QRCodeError::SpecifiedVersion => {
                f.write_str("Specified version too low to contain data")
            }
        }
    }
}

impl QRCode {
    /// Creates a new `QRCode` from a ECL / version
    ///
    /// # Errors
    /// - `QRCodeError::EncodedData` if `input` is too large to be encoded
    /// - `QRCodeError::SpecifiedVersion` if specified `version` is too small to contain data
    pub(crate) fn new(
        input: &[u8],
        ecl: Option<ECL>,
        v: Option<Version>,
        mut mask: Option<Mask>,
    ) -> Result<Self, QRCodeError> {
        use crate::placement::create_matrix;

        let mode = encode::best_encoding(input);
        let level = ecl.unwrap_or(ECL::Q);

        let version = match Version::get(mode, level, input.len()) {
            Some(version) => version,
            None => return Err(QRCodeError::EncodedData),
        };
        let version = match v {
            Some(user_version) if user_version as usize >= version as usize => user_version,
            None => version,
            Some(_) => return Err(QRCodeError::SpecifiedVersion),
        };

        let out = create_matrix(input, level, mode, version, &mut mask);
        Ok(out)
    }

    #[cfg(not(target_arch = "wasm32"))]
    /// Prints the `QRCode` to the terminal
    #[must_use]
    pub fn to_str(&self) -> String {
        helpers::print_matrix_with_margin(self)
    }

    #[cfg(not(target_arch = "wasm32"))]
    /// Prints the `QRCode` to the terminal
    pub fn print(&self) {
        println!("{}", helpers::print_matrix_with_margin(self));
    }
}

/// Builder struct, makes it easier to create a [`QRCode`].
///
/// # Example
/// ```rust
/// use fast_qr::QRBuilder;
/// use fast_qr::{Mask, ECL, Version};
///
/// // Creates a `QRCode` with a forced `version`, `ecl` and/or `mask`
/// let input = String::from("Hello World!");
/// let qr = QRBuilder::new(input)
///     // .version(Version::V05)
///     // .ecl(ECL::H)
///     // .mask(Mask::Checkerboard)
///     .build();
/// ```
pub struct QRBuilder {
    input: Vec<u8>,
    ecl: Option<ECL>,
    // mode: Option<Mode>,
    version: Option<Version>,
    mask: Option<Mask>,
}

impl QRBuilder {
    /// Creates an instance of `QRBuilder` with default parameters
    #[must_use]
    pub fn new<I: Into<Vec<u8>>>(input: I) -> QRBuilder {
        QRBuilder {
            input: input.into(),
            mask: None,
            // mode: None,
            version: None,
            ecl: None,
        }
    }

    // pub fn mode(&mut self, mode: Mode) -> &mut Self {
    //     self.mode = Some(mode);
    //     self
    // }

    /// Forces the Encoding Level
    pub fn ecl(&mut self, ecl: ECL) -> &mut Self {
        self.ecl = Some(ecl);
        self
    }

    /// Forces the version
    pub fn version(&mut self, version: Version) -> &mut Self {
        self.version = Some(version);
        self
    }

    /// Forces the mask, should very rarely be used
    pub fn mask(&mut self, mask: Mask) -> &mut Self {
        self.mask = Some(mask);
        self
    }

    /// Computes a [`QRCode`] with given parameters
    ///
    /// # Errors
    /// - `QRCodeError::EncodedData` if `input` is too large to be encoded. See [an online table](https://fast-qr.com/blog/tables/ecl) for more info.
    /// - `QRCodeError::SpecifiedVersion` if specified `version` is too small to contain data
    pub fn build(&self) -> Result<QRCode, QRCodeError> {
        QRCode::new(&self.input, self.ecl, self.version, self.mask)
    }
}
