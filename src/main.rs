use std::env;

mod ihexparse;
mod emulator;
mod instruction;

fn main() {
  let filename = &env::args().last().expect("Filename isn't specified");
  let insts = ihexparse::parse_file(filename);
  println!("File parsed: {:?}", insts);
  return;

  // let emu = emulator::Emulator::new();
  // emu.load(insts);
  // emu.execute();
}