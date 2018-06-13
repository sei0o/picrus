struct Emulator {
  program_mem: Box<[u16]>,
  data_mem: Box<[u8]>,
  file_reg: Box<[u8]>,
  w_reg: u8,
  pc: u16,
}

impl Emulator {
  fn new() -> Emulator {
    Emulator {
      program_mem: Box::new(1024),
      data_mem: Box::new(64),
      file_reg: Box::new(256),
      w_reg: 0,
      pc: 0,
    }
  }
}