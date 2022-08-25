use barcoders::{generators::image::Image, sym::code39::Code39};
use regex::Regex;
use std::{
    fs::File,
    io::{BufWriter, Write},
    path::Path,
    process::Command,
};

fn main() {
    let command_output = get_sn_code();
    let sn_code = match_sn_code(command_output);
    let barcode_png = sn_code_2_barcode_png(sn_code);
    save_png_file(barcode_png);
}

fn save_png_file(png: Vec<u8>) {
    let file = File::create(&Path::new("SNCode.png")).unwrap();
    let mut writer = BufWriter::new(file);
    writer.write(&png[..]).unwrap();
}

fn get_sn_code() -> String {
    let output = Command::new("wmic.exe")
        .arg("bios")
        .arg("get")
        .arg("serialnumber")
        .output()
        .unwrap();
    String::from_utf8(output.stdout).unwrap()
}

fn match_sn_code(command_output: String) -> String {
    let re = Regex::new(r"(\S{10})").unwrap();
    for (n, sn) in re.captures_iter(&command_output).enumerate() {
        if n == 1 {
            return sn.get(1).unwrap().as_str().to_string();
        }
    }
    String::new()
}

fn sn_code_2_barcode_png(sn_code: String) -> Vec<u8> {
    let barcode = Code39::new(sn_code).unwrap();
    let png = Image::png(80); // You must specify the height in pixels.
    let encoded = barcode.encode();

    // Image generators return a Result<Vec<u8>, barcoders::error::Error) of encoded bytes.
    png.generate(&encoded[..]).unwrap()
}
