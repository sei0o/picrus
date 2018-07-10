use emulator::Emulator;

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
  match d { 
    0 => emu.w_reg += fval,
    1 => emu.set_file_reg(f, wval + fval),
    _ => panic!("Expected 0 or 1")
  }
  // TODO: change status C, DC, Z
  emu.pc += 1;
}

pub fn clrf(emu: &mut Emulator) {
  let instr = emu.program_mem[emu.pc as usize];
  let f = instr & 0x7f;
  emu.set_file_reg(f as usize, 0);
  emu.set_z_bit(1);
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

pub fn movlw(emu: &mut Emulator) {
  let instr = emu.program_mem[emu.pc as usize];
  let k: u8 = (instr & 0xff) as u8;
  emu.w_reg = k;
  emu.pc += 1;
}

// Return from subroutine
pub fn ret(emu: &mut Emulator) {
  emu.pc = emu.stack.pop().expect("Found RETURN operation but the stack is empty");
} 