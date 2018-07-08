use emulator::Emulator;

//
// Byte-oriented file register operations
//

pub fn clrf(emu: &mut Emulator) {
  let instr = emu.program_mem[emu.pc as usize];
  let f = instr & 0x7f;
  emu.file_reg[f as usize] = 0;
  emu.set_z_bit(1);
  emu.pc += 1;
}

pub fn nop(emu: &mut Emulator) {
}

//
// Bit-oriented file register operations
// 

// Bit clear f
pub fn bcf(emu: &mut Emulator) {
  let instr = emu.program_mem[emu.pc as usize];
  let b = (instr >> 7) & 0x7;
  let f = instr & 0x7f;
  emu.file_reg[f as usize] &= 1 << b;
  emu.pc += 1;
}

// Bit set f
pub fn bsf(emu: &mut Emulator) {
  let instr = emu.program_mem[emu.pc as usize];
  let b = (instr >> 7) & 0x7;
  let f = instr & 0x7f;
  emu.file_reg[f as usize] &= 1 << b;
  emu.pc += 1;
}

//
// Literal and control operations
//

// Unconditional branch
pub fn goto(emu: &mut Emulator) {
  let instr = emu.program_mem[emu.pc as usize];
  // TODO: get upper bits from PCLATH(SFR) 
  let pclath_addr = 0;
  emu.pc = (pclath_addr << 11) | (instr & 0x7ff);
}