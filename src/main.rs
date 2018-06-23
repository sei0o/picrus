use std::env;

mod ihexparse;
mod register;
mod emulator;
mod instruction;

fn main() {
  let filename = &env::args().last().expect("Filename isn't specified");
  let insts = ihexparse::parse_file(filename);
  println!("File parsed: {:?}", insts);

  let mut emu = emulator::Emulator::new();
  emu.load(insts.as_slice());
  emu.execute();
}