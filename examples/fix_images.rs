use std::path::Path;

use image::{ImageFormat, ImageReader};

fn main() {
    const TYPES_SINGLE: &[(&str, &str)] = &[
        ("currency", "currency"),
        ("gem", "gem"),
        ("magic", "magic"),
        ("quest", "quest"),
        ("raresingleline", "rare"),
        ("uniquesingleline", "unique"),
        ("white", "normal"),
    ];
    const TYPES_DOUBLE: &[(&str, &str)] = &[
        ("doublecurrency", "doublecurrency"),
        ("doublegem", "doublegem"),
        ("doublemagic", "doublemagic"),
        ("doublenormal", "doublenormal"),
        ("doublequest", "doublequest"),
        ("rare", "doublerare"),
        ("unique", "doubleunique"),
    ];
    const SIDES: &[&str] = &[
        "left", "middle", "right"
    ];

    let in_dir = Path::new("assets/4k");
    let out_dir = Path::new("assets");

    for (in_imagetype, out_imagetype) in TYPES_SINGLE {
        for side in SIDES {
            let filename = format!("itemsheader{in_imagetype}{side}.png");
            let in_fullpath = in_dir.join(&filename);
            let filename = format!("itemsheader{out_imagetype}{side}.png");
            let out_fullpath = out_dir.join(&filename);
            let mut img = ImageReader::open(in_fullpath).unwrap().decode().unwrap();
            let out_img = img.crop(0, 0, 63, 74);
            out_img.write_to(&mut std::fs::File::create(out_fullpath).unwrap(), ImageFormat::Png).unwrap();
        }
    }

    for (in_imagetype, out_imagetype) in TYPES_DOUBLE {
        for side in SIDES {
            let filename = format!("itemsheader{in_imagetype}{side}.png");
            let in_fullpath = in_dir.join(&filename);
            let filename = format!("itemsheader{out_imagetype}{side}.png");
            let out_fullpath = out_dir.join(&filename);
            let mut img = ImageReader::open(in_fullpath).unwrap().decode().unwrap();
            let out_img = img.crop(0, 0, 96, 119);
            out_img.write_to(&mut std::fs::File::create(out_fullpath).unwrap(), ImageFormat::Png).unwrap();
        }
    }
}