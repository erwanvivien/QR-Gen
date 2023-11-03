fn main() {
    use fast_qr::{
        convert::{svg::SvgBuilder, Builder, EyeFrameShape, ModuleShape},
        QRBuilder, Version, ECL,
    };

    let qrcode = QRBuilder::new("https://example.com/")
        .ecl(ECL::H)
        .version(Version::V03)
        .build()
        .unwrap();

    let _svg = SvgBuilder::default()
        .module_shape(ModuleShape::RoundedSquare)
        .eye_frame_shape(EyeFrameShape::Square)
        .to_file(&qrcode, "svg.svg");
}
