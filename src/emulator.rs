use instruction;
use register;
use register::{bank0, bank1};

pub struct Emulator {
  pub program_mem: Vec<u16>,
  pub data_mem: Vec<u8>,
  pub file_reg: Vec<u8>,
  pub w_reg: u8,
  pub pc: u16,
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

  pub fn do_next_instruction(&mut self) { 
    match self.program_mem[self.pc as usize] {
      // Byte-oriented file register operations
      op if (op >> 7) == 0b0000011 => instruction::clrf(self),

      // Bit-oriented file register operations
      op if (op >> 10) == 0b0100 => instruction::bcf(self),
      op if (op >> 10) == 0b0101 => instruction::bsf(self),
      
      // Literal and control operations
      op if (op >> 11) == 0b101 => instruction::goto(self),
      
      _ => {
        println!("Not Implemented");
        self.pc += 1;
      },
    }
  }

  pub fn get_sfr(&self, addr: usize) -> u8 {
    self.file_reg[addr]
  }

  pub fn set_sfr(&mut self, addr: usize, val: u8) {
    self.file_reg[addr] = val;
    match register::pair_for(addr) {
      Some(pair) => self.file_reg[pair] = val,
      None => return
    }
  }

  pub fn get_z_bit(&self) -> u8 {
    (self.get_sfr(bank0::STATUS) >> 2) & 1
  }

  pub fn set_z_bit(&mut self, z: u8) {
    let old_status = self.get_sfr(bank0::STATUS);
    self.set_sfr(bank0::STATUS, old_status & !(1 << 2) | (z << 2));
  }
}