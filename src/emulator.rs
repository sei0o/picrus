pub struct Emulator {
  program_mem: Vec<u16>,
  data_mem: Vec<u8>,
  file_reg: Vec<u8>,
  w_reg: u8,
  pc: u16,
}

impl Emulator {
  pub fn new() -> Emulator {
    Emulator {
      program_mem: vec![0; 1024],
      data_mem: vec![0; 64],
      file_reg: vec![0; 256],
      w_reg: 0,
      pc: 0,
    }
  }

  pub fn load(&mut self, insts: &[u16]) {
    for (i, inst) in insts.iter().enumerate() {
      self.program_mem[i] = *inst;
    }
  }

  pub fn execute(&mut self) {
    while self.pc < 1024 {
      self.do_next_instruction();
    }
  }

  pub fn dump_regs(&self) {

  }

  pub fn do_next_instruction(&self) { 
    match self.program_mem[self.pc as usize] {
      _ => println!("Not Implemented"),
    }
  }
}