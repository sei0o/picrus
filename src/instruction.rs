use emulator::Emulator;

pub fn nop(emu: &mut Emulator) {
}

// Unconditional branch
pub fn goto(emu: &mut Emulator) {
  let instr = emu.program_mem[emu.pc as usize];
  // TODO: get upper bits from PCLATH(SFR) 
  let pclath_addr = 0;
  emu.pc = (pclath_addr << 11) | (instr & 0x7ff);
}