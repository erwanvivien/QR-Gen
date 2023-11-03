fn main() {
    use fast_qr::{
        convert::{image::ImageBuilder, Builder, ModuleShape},
        ModuleType, QRBuilder, Version, ECL,
    };

    let qrcode = QRBuilder::new("https://example.com/")
        .ecl(ECL::H)
        .version(Version::V03)
        .build()
        .unwrap();

    let mut _img = ImageBuilder::default()
        // Can have many shapes and custom shapes
        .module_shape(ModuleShape::Command(|y, x, cell| {
            match cell.module_type() {
                ModuleType::FinderPattern | ModuleType::Alignment => String::new(),
                _ => {
                    // Works thanks to Deref
                    ModuleShape::Square(y, x, cell)
                }
            }
        }))
        .module_shape_color(
            ModuleShape::Command(|y, x, cell| {
                match cell.module_type() {
                    ModuleType::FinderPattern | ModuleType::Alignment => {
                        // Works thanks to Deref
                        ModuleShape::Circle(y, x, cell)
                    }
                    _ => String::new(),
                }
            }),
            [255, 0, 0, 255],
        )
        .fit_width(600)
        .background_color([255, 255, 255, 255])
        .to_file(&qrcode, "custom.png");
}
