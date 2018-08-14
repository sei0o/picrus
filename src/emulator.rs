use instruction;
use register;
use register::{bank0, bank1};

pub struct Emulator {
  pub program_mem: Vec<u16>,
  pub data_mem: Vec<u8>,
  pub file_reg: Vec<u8>,
  pub w_reg: u8,
  pub pc: u16,
  pub stack: Vec<u16>,
}

impl Emulator {
  pub fn new() -> Emulator {
    Emulator {
      program_mem: vec![0; 1024],
      data_mem: vec![0; 64],
      file_reg: vec![0; 256],
      w_reg: 0,
      pc: 0,
      stack: vec![0; 8],
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
    println!("PC: {}", self.pc);
    println!("W register: {}", self.w_reg);
    
    println!("INDF: {}", self.file_reg[0x00 as usize]);
    println!("TMR0: {}", self.file_reg[0x01 as usize]);
    println!("PCL: {}", self.file_reg[0x02 as usize]);
    println!("STATUS: {}, Z: {}, DC: {}, C: {}", self.file_reg[0x03 as usize], self.get_z_bit(), self.get_dc_bit(), self.get_c_bit());
    println!("FSR: {}", self.file_reg[0x04 as usize]);
    println!("PORTA: {}", self.file_reg[0x05 as usize]);
    println!("PORTB: {}", self.file_reg[0x06 as usize]);
    println!("EEDATA: {}", self.file_reg[0x08 as usize]);
    println!("EEADR: {}", self.file_reg[0x09 as usize]);
    println!("PCLATH: {}", self.file_reg[0x0a as usize]);
    println!("INTCON: {}", self.file_reg[0x0b as usize]);

    println!("OPTION_REG: {}", self.file_reg[0x81 as usize]);
    println!("TRISA: {}", self.file_reg[0x85 as usize]);
    println!("TRISB: {}", self.file_reg[0x86 as usize]);
    println!("EECON1: {}", self.file_reg[0x88 as usize]);
    println!("EECON2: {}", self.file_reg[0x89 as usize]);
  }

  pub fn do_next_instruction(&mut self) { 
    match self.program_mem[self.pc as usize] {
      // Byte-oriented file register operations
      op if (op >> 8) == 0b000111 => instruction::addwf(self),
      op if (op >> 8) == 0b000101 => instruction::andwf(self),
      op if (op >> 7) == 0b0000011 => instruction::clrf(self),
      op if (op >> 7) == 0b0000010 => instruction::clrw(self),
      op if (op >> 8) == 0b001001 => instruction::comf(self),
      op if (op >> 8) == 0b000011 => instruction::decf(self),
      op if (op >> 8) == 0b001011 => instruction::decfsz(self),
      op if (op >> 8) == 0b001010 => instruction::incf(self),
      op if (op >> 8) == 0b001111 => instruction::incfsz(self),
      op if (op >> 8) == 0b000100 => instruction::iorwf(self),
      op if (op >> 8) == 0b001000 => instruction::movf(self),
      op if (op >> 7) == 0b0000001 => instruction::movwf(self),
      op if (op >> 7) == 0b0000000 && (op & 0b11111) == 0 => instruction::nop(self),
      op if (op >> 8) == 0b000010 => instruction::subwf(self),
      op if (op >> 8) == 0b001110 => instruction::swapf(self),
      op if (op >> 8) == 0b000110 => instruction::xorwf(self),

      // Bit-oriented file register operations
      op if (op >> 10) == 0b0100 => instruction::bcf(self),
      op if (op >> 10) == 0b0101 => instruction::bsf(self),
      
      // Literal and control operations
      op if (op >> 9) == 0b11111 => instruction::addlw(self),
      op if (op >> 8) == 0b111001 => instruction::andlw(self),
      op if (op >> 11) == 0b100 => instruction::call(self),
      op if (op >> 11) == 0b101 => instruction::goto(self),
      op if (op >> 8) == 0b111000 => instruction::iorlw(self),
      op if (op >> 10) == 0b1100 => instruction::movlw(self),
      0b00_0000_0000_1001 => instruction::retfie(self),
      op if (op >> 10) == 0b1101 => instruction::retlw(self),
      0b00_0000_0000_1000 => instruction::ret(self),
      op if (op >> 9) == 0b11110 => instruction::sublw(self),
      op if (op >> 8) == 0b111010 => instruction::xorlw(self),
      
      _ => {
        println!("Not Implemented");
        self.pc += 1;
      },
    }
  }

  pub fn get_file_reg(&self, addr: usize) -> u8 {
    self.file_reg[addr]
  }

  pub fn set_file_reg(&mut self, addr: usize, val: u8) {
    self.file_reg[addr] = val;
    match register::pair_for(addr) {
      Some(pair) => self.file_reg[pair] = val,
      None => return
    }
  }

  pub fn get_z_bit(&self) -> u8 {
    (self.get_file_reg(bank0::STATUS) >> 2) & 1
  }

  pub fn set_z_bit(&mut self, z: u8) {
    let old_status = self.get_file_reg(bank0::STATUS);
    self.set_file_reg(bank0::STATUS, (old_status & !(1 << 2)) | (z << 2));
  }

  pub fn get_dc_bit(&self) -> u8 {
    (self.get_file_reg(bank0::STATUS) >> 1) & 1
  }

  pub fn set_dc_bit(&mut self, dc: u8) {
    let old_status = self.get_file_reg(bank0::STATUS);
    self.set_file_reg(bank0::STATUS, (old_status & !(1 << 1)) | (dc << 1));
  }

  pub fn get_c_bit(&self) -> u8 {
    (self.get_file_reg(bank0::STATUS)) & 1
  }

  pub fn set_c_bit(&mut self, c: u8) {
    let old_status = self.get_file_reg(bank0::STATUS);
    self.set_file_reg(bank0::STATUS, (old_status & !0) | c);
  }
}