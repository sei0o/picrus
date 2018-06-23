// Special Functions Register (SFR)
pub mod bank0 {
  pub const INDF:   usize = 0x00;
  pub const TMR0:   usize = 0x01;
  pub const PCL:    usize = 0x02;
  pub const STATUS: usize = 0x03;
  pub const FSR:    usize = 0x04;
  pub const PORTA:  usize = 0x05;
  pub const PORTB:  usize = 0x06;
  // 0x07 is unimplemented
  pub const EEDATA: usize = 0x08;
  pub const EEADR:  usize = 0x09;
  pub const PCLATH: usize = 0x0a;
  pub const INTCON: usize = 0x0b;
}

pub mod bank1 {
  pub const INDF:       usize = 0x80;
  pub const OPTION_REG: usize = 0x81;
  pub const PCL:        usize = 0x82;
  pub const STATUS:     usize = 0x83;
  pub const FSR:        usize = 0x84;
  pub const TRISA:      usize = 0x85;
  pub const TRISB:      usize = 0x86;
  // 0x87 is unimplemented
  pub const EECON1:     usize = 0x88;
  pub const EECON2:     usize = 0x89;
  pub const PCLATH:     usize = 0x8a;
  pub const INTCON:     usize = 0x8b;
}

// Returns register address which has the same name in the other bank
pub fn pair_for(reg: usize) -> Option<usize> {
  match reg {
    // bank 0 -> bank 1
    0x00 => Some(0x80),
    0x01 => None,
    0x02 => Some(0x82),
    0x03 => Some(0x83),
    0x04 => Some(0x84),
    0x05 => None,
    0x06 => None,
    0x08 => None,
    0x09 => None,
    0x0a => Some(0x8a),
    0x0b => Some(0x8b),
    // bank 1 -> bank 0
    0x80 => Some(0x00),
    0x81 => None,
    0x82 => Some(0x02),
    0x83 => Some(0x03),
    0x84 => Some(0x04),
    0x85 => None,
    0x86 => None,
    0x88 => None,
    0x89 => None,
    0x8a => Some(0x0a),
    0x8b => Some(0x0b),
    _ => panic!("Unknown register number")
  }
}