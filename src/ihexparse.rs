use std::fs::File;
use std::io::*;
use std::str;

pub fn parse_file(filename: &str) -> Vec<u16> {
  // see: https://stackoverflow.com/questions/47660946/why-does-a-file-need-to-be-mutable-to-call-readread-to-string
  let mut f = File::open(filename).expect("File Not Found");
  let mut content = String::new();
  f.read_to_string(&mut content).expect("Couldn't read file");

  let payload_hex = content[1..content.len()-1] // remove last empty line
    .split('\n')
    .filter(|x| &x[7..9] == "00")
    .map(|x| &x[9..x.len()-2])
    .collect::<Vec<&str>>()
    .join("");
    
  let payload_bin: Vec<u16> = payload_hex
    .as_bytes()
    .chunks(4)
    .map(|chunk| {
      let st = str::from_utf8(chunk).unwrap();
      (u8::from_str_radix(&st[2..=3], 16).unwrap() as u16) << 8 | (u8::from_str_radix(&st[0..=1], 16).unwrap() as u16)
    })
    .collect();

  payload_bin
}