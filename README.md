<div style="display: flex; justify-content: center">
  <img src="assets/banner.svg"  alt="Example qr for website example.com"/>
</div>

`fast_qr` is approximately 6-7 times faster than `qrcode`, see [benchmarks](#benchmarks)

You can create a QR as

- [x] Raw matrix, well suited for custom usage
- [x] Vectorized image, well suited for web usage
- [x] Image, well suited for mobile / print usage

### Usage

## Converts `QRCode` to Unicode

```rust
use fast_qr::convert::ConvertError;
use fast_qr::convert::{svg::SvgBuilder, Builder, Shape};
use fast_qr::qr::QRBuilder;

fn main() -> Result<(), ConvertError> {
    // QRBuilde::new can fail if content is too big for version,
    // please check before unwrapping.
    let qrcode = QRBuilder::new("https://example.com/".into())
        .build()
        .unwrap();

    let str = qrcode.to_str(); // .print() exists
    println!("{}", str);

    Ok(())
}
```

## Converts `QRCode` to SVG [docs.rs](https://docs.rs/fast_qr/0.6.0/fast_qr/convert/svg/index.html)

```rust
use fast_qr::convert::ConvertError;
use fast_qr::convert::{svg::SvgBuilder, Builder, Shape};
use fast_qr::qr::QRBuilder;

fn main() -> Result<(), ConvertError> {
    // QRBuilde::new can fail if content is too big for version,
    // please check before unwrapping.
    let qrcode = QRBuilder::new("https://example.com/".into())
        .build()
        .unwrap();

    let _svg = SvgBuilder::default()
        .shape(Shape::RoundedSquare)
        .to_file(&qrcode, "out.svg");

    Ok(())
}
```

## Converts `QRCode` to an image [docs.rs](https://docs.rs/fast_qr/0.6.0/fast_qr/convert/image/index.html)

```rust
use fast_qr::convert::ConvertError;
use fast_qr::convert::{image::ImageBuilder, Builder, Shape};
use fast_qr::qr::QRBuilder;

fn main() -> Result<(), ConvertError> {
    // QRBuilde::new can fail if content is too big for version,
    // please check before unwrapping.
    let qrcode = QRBuilder::new("https://example.com/".into())
        .build()
        .unwrap();

    let _img = ImageBuilder::default()
        .shape(Shape::RoundedSquare)
        .fit_width(600)
        .to_file(&qrcode, "out.png");

    Ok(())
}
```

## Build WASM

### WASM module also exists in NPM registry

Package is named `fast_qr` and can be installed like so :

```
npm install --save fast_qr
```

### WASM module might be bundled

Find a bundled version in the latest [release](https://github.com/erwanvivien/fast_qr/releases).

### WASM module can be built from source

```bash
wasm-pack build --target web # All ready in ./pkg
wasm-opt -Os -o pkg/fast_qr_bg.wasm pkg/fast_qr_bg.wasm # Optimizes wasm module size
wasm-pack pack pkg # Generates the package to be published
wasm-pack publish # you might need to `npm login`
```

Note: I found that wasm-opt doesn't always work, so I download the binary from
[WebAssembly/binaryen](https://github.com/WebAssembly/binaryen).

## Benchmarks

According to the following benchmarks, `fast_qr` is approximately 6-7x faster than `qrcode`.

| Benchmark    |   Lower   | Estimate  |   Upper   |                         |
| :----------- | :-------: | :-------: | :-------: | ----------------------- |
| V03H/qrcode  | 524.30 us | 535.02 us | 547.13 us |                         |
| V03H/fast_qr | 82.079 us | 82.189 us | 82.318 us | fast_qr is 6.51x faster |
| V10H/qrcode  | 2.1105 ms | 2.1145 ms | 2.1186 ms |                         |
| V10H/fast_qr | 268.70 us | 269.28 us | 269.85 us | fast_qr is 7.85x faster |
| V40H/qrcode  | 18.000 ms | 18.037 ms | 18.074 ms |                         |
| V40H/fast_qr | 2.4313 ms | 2.4362 ms | 2.4411 ms | fast_qr is 7.40x faster |

More benchmarks can be found in [/benches folder](https://github.com/erwanvivien/fast_qr/tree/master/benches).
