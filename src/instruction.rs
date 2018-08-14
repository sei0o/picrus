use emulator::Emulator;
use register;
use register::{bank0, bank1};

//
// Byte-oriented file register operations
//

// Add W and f
pub fn addwf(emu: &mut Emulator) {
  let instr = emu.program_mem[emu.pc as usize];
  let f = (instr & 0x7f) as usize;
  let d = (instr >> 7) & 1;
  let fval = emu.get_file_reg(f);
  let wval = emu.w_reg;
  let sum = (fval as u16) + (wval as u16);
  emu.set_z_bit((sum == 0) as u8);
  emu.set_c_bit((sum >= 0x100) as u8);
  match d { 
    0 => {
      emu.w_reg += fval;
      emu.set_dc_bit(((wval < 0x10) && (sum > 0x10)) as u8);
    },
    1 => {    
      emu.set_file_reg(f, wval + fval);
      emu.set_dc_bit(((fval < 0x10) && (sum > 0x10)) as u8); // ? compare with gpsim's put_Z_C_DC_OV_N()
    },
    _ => panic!("Expected 0 or 1")
  }
  emu.pc += 1;
}

// AND W with f
pub fn andwf(emu: &mut Emulator) {
  let instr = emu.program_mem[emu.pc as usize];
  let f = (instr & 0x7f) as usize;
  let d = (instr >> 7) & 1;
  let fval = emu.get_file_reg(f);
  let wval = emu.w_reg;
  
  let result = fval & wval;
  emu.set_z_bit((result == 0) as u8);
  match d {
    0 => emu.w_reg = result,
    1 => emu.set_file_reg(f, result),
    _ => panic!("Expected 0 or 1")
  }
  emu.pc += 1;
}

pub fn clrf(emu: &mut Emulator) {
  let instr = emu.program_mem[emu.pc as usize];
  let f = instr & 0x7f;
  emu.set_file_reg(f as usize, 0);
  emu.set_z_bit(1);
  emu.pc += 1;
}

// Clear W
pub fn clrw(emu: &mut Emulator) {
  emu.w_reg = 0;
  emu.set_z_bit(1);
  emu.pc += 1;
}

// Complement f
pub fn comf(emu: &mut Emulator) {
  let instr = emu.program_mem[emu.pc as usize];
  let f = (instr & 0x7f) as usize;
  let d = (instr >> 7) & 1;
  let fval = emu.get_file_reg(f);
  emu.set_z_bit((!fval == 0) as u8);
  match d {
    0 => emu.w_reg = !fval,
    1 => emu.set_file_reg(f, !fval),
    _ => panic!("Expected 0 or 1")
  }

  emu.pc += 1;
}

// Decrement f
pub fn decf(emu: &mut Emulator) {
  let instr = emu.program_mem[emu.pc as usize];
  let f = (instr & 0x7f) as usize;
  let d = (instr >> 7) & 1;
  let result = emu.get_file_reg(f).wrapping_sub(1);

  emu.set_z_bit((result == 0) as u8);
  match d {
    0 => emu.w_reg = result,
    1 => emu.set_file_reg(f, result),
    _ => panic!("Expected 0 or 1")
  }

  emu.pc += 1;
}

// Decrement f, Skip if 0
pub fn decfsz(emu: &mut Emulator) {
  let instr = emu.program_mem[emu.pc as usize];
  let f = (instr & 0x7f) as usize;
  let d = (instr >> 7) & 1;
  let result = emu.get_file_reg(f).wrapping_sub(1);

  match d {
    0 => emu.w_reg = result,
    1 => emu.set_file_reg(f, result),
    _ => panic!("Expected 0 or 1")
  }

  emu.pc += match result {
    0 => 2,
    _ => 1,
  }
}

// Increment f
pub fn incf(emu: &mut Emulator) {
  let instr = emu.program_mem[emu.pc as usize];
  let f = (instr & 0x7f) as usize;
  let d = (instr >> 7) & 1;
  let result = emu.get_file_reg(f).wrapping_add(1);

  emu.set_z_bit((result == 0) as u8);
  match d {
    0 => emu.w_reg = result,
    1 => emu.set_file_reg(f, result),
    _ => panic!("Expected 0 or 1")
  }

  emu.pc += 1;
}

// Increment f, Skip if 0
pub fn incfsz(emu: &mut Emulator) {
  let instr = emu.program_mem[emu.pc as usize];
  let f = (instr & 0x7f) as usize;
  let d = (instr >> 7) & 1;
  let result = emu.get_file_reg(f).wrapping_add(1);

  emu.set_z_bit((result == 0) as u8);
  match d {
    0 => emu.w_reg = result,
    1 => emu.set_file_reg(f, result),
    _ => panic!("Expected 0 or 1")
  }

  emu.pc += match result {
    0 => 2,
    _ => 1
  }
}

// Inclusive OR W with f
pub fn iorwf(emu: &mut Emulator) {
  let instr = emu.program_mem[emu.pc as usize];
  let f = (instr & 0x7f) as usize;
  let d = (instr >> 7) & 1;
  let result = emu.w_reg | emu.get_file_reg(f);

  emu.set_z_bit((result == 0) as u8);
  match d {
    0 => emu.w_reg = result,
    1 => emu.set_file_reg(f, result),
    _ => panic!("Expected 0 or 1")
  }

  emu.pc += 1;
}

pub fn movf(emu: &mut Emulator) {
  let instr = emu.program_mem[emu.pc as usize];
  let f = (instr & 0x7f) as usize;
  let d = (instr >> 7) & 1;
  let fval = emu.get_file_reg(f);

  emu.set_z_bit((fval == 0) as u8);
  match d {
    0 => emu.w_reg = fval,
    1 => emu.set_file_reg(f, fval), // ? or do nothing
    _ => panic!("Expected 0 or 1")
  }

  emu.pc += 1;
}

pub fn movwf(emu: &mut Emulator) {
  let instr = emu.program_mem[emu.pc as usize];
  let f = instr & 0x7f;
  let w = emu.w_reg;
  emu.set_file_reg(f as usize, w);

  emu.pc += 1;
}

pub fn nop(emu: &mut Emulator) {
  emu.pc += 1;
}

//
// Bit-oriented file register operations
// 

// Bit clear f
pub fn bcf(emu: &mut Emulator) {
  let instr = emu.program_mem[emu.pc as usize];
  let b = (instr >> 7) & 0x7;
  let f = (instr & 0x7f) as usize;
  let val = emu.get_file_reg(f);
  emu.set_file_reg(f, val & (1 << b));
  emu.pc += 1;
}

// Bit set f
pub fn bsf(emu: &mut Emulator) {
  let instr = emu.program_mem[emu.pc as usize];
  let b = (instr >> 7) & 0x7;
  let f = (instr & 0x7f) as usize;
  let val = emu.get_file_reg(f);
  emu.set_file_reg(f, val & (1 << b));
  emu.pc += 1;
}

//
// Literal and control operations
//

// Call subroutine
pub fn call(emu: &mut Emulator) {
  let instr = emu.program_mem[emu.pc as usize];
  
  emu.stack.push(emu.pc + 1);

  // TODO: get upper bits from PCLATH
  let pclath_addr = 0;
  emu.pc = (pclath_addr << 11) | (instr & 0x7ff);
}

// Unconditional branch
pub fn goto(emu: &mut Emulator) {
  let instr = emu.program_mem[emu.pc as usize];

  if instr & 0x7ff == 0x400 {
    emu.dump_regs();
    emu.pc += 1;
    return;
  }

  // TODO: get upper bits from PCLATH(SFR) 
  let pclath_addr = 0;
  emu.pc = (pclath_addr << 11) | (instr & 0x7ff);
}

// Inclusive OR literal with W
pub fn iorlw(emu: &mut Emulator) {
  let instr = emu.program_mem[emu.pc as usize];
  let k: u8 = (instr & 0xff) as u8;
  let result = emu.w_reg | k;
  emu.w_reg = result;
  emu.set_z_bit((result == 0) as u8);
  emu.pc += 1;
}

pub fn movlw(emu: &mut Emulator) {
  let instr = emu.program_mem[emu.pc as usize];
  let k: u8 = (instr & 0xff) as u8;
  emu.w_reg = k;
  emu.pc += 1;
}

// Return from interrupt
pub fn retfie(emu: &mut Emulator) {
  let old_intcon = emu.get_file_reg(bank0::INTCON);
  emu.set_file_reg(bank0::INTCON, (old_intcon & !0x80) | 1);
  emu.pc = emu.stack.pop().expect("Found RETFIE operation but the stack is empty");
}

// Return with literal in W register
pub fn retlw(emu: &mut Emulator) {
  let instr = emu.program_mem[emu.pc as usize];
  let k: u8 = (instr & 0xff) as u8;
  emu.w_reg = k;
  emu.pc = emu.stack.pop().expect("Found RETLW operation but the stack is empty");
}

// Return from subroutine
pub fn ret(emu: &mut Emulator) {
  emu.pc = emu.stack.pop().expect("Found RETURN operation but the stack is empty");
} 