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