struct Rgb(u8, u8, u8);

pub fn struct_tuple_func() {
    println!("==================== rgb ==================");
    let rgb = Rgb(255, 255, 14);
    println!("rgb: {}.{}.{}", rgb.0, rgb.1, rgb.2);

    println!("==================== rgb to hex ==================");
    println!("hex: {}", to_hex(rgb).to_string());
}

fn to_hex(rgb: Rgb) -> String {
    format!("0x{}", format!("{:02X}{:02X}{:02X}", rgb.0, rgb.1, rgb.2))
}
