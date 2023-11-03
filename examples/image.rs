fn main() {
    use fast_qr::{
        convert::{image::ImageBuilder, Builder, ModuleShape},
        QRBuilder, Version, ECL,
    };

    let qrcode = QRBuilder::new("https://example.com/")
        .ecl(ECL::H)
        .version(Version::V03)
        .build()
        .unwrap();

    let _image = ImageBuilder::default()
        .module_shape(ModuleShape::RoundedSquare)
        .fit_width(600)
        .background_color([255, 255, 255, 0]) // transparency
        .to_file(&qrcode, "image.png");

    // Or maybe as bytes.
    let _image_as_bytes = ImageBuilder::default()
        .module_shape(ModuleShape::RoundedSquare)
        .fit_width(512)
        .background_color([255, 255, 255, 255]) // opaque
        .to_bytes(&qrcode);
}
