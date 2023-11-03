fn main() {
    use fast_qr::{
        convert::{image::ImageBuilder, Builder, ImageBackgroundShape, ModuleShape},
        QRBuilder, Version, ECL,
    };

    let qrcode = QRBuilder::new("https://example.com/")
        .ecl(ECL::H)
        .version(Version::V03)
        .build()
        .unwrap();

    let mut _img = ImageBuilder::default()
        .module_shape(ModuleShape::Square)
        .fit_width(600)
        .background_color([255, 255, 255, 255])
        // New: embed an image
        .image(String::from("./assets/example.com.svg"))
        // .image_size(15f64, 2f64)
        // .image_position(37f64 / 2f64, 0f64)
        .image_background_color([165, 34, 247, 255])
        .image_background_shape(ImageBackgroundShape::Square)
        .to_file(&qrcode, "embed.png");
}
