//! Module for things related to ROM.

use super::*;

pub const WAITCNT: VolAddress<WaitstateControl> = unsafe { VolAddress::new(0x400_0204) };

newtype!(WaitstateControl, pub u32);

impl WaitstateControl {
  phantom_fields! {
    self.0: u32,
    sram: 0-1 = WaitstateFirstAccess<Cycles4, Cycles3, Cycles2, Cycles8>,
    ws0_first_access: 2-3 = WaitstateFirstAccess<Cycles4, Cycles3, Cycles2, Cycles8>,
    ws0_second_access: 4, // true = 2, false = 1
    ws1_first_access: 5-6 = WaitstateFirstAccess<Cycles4, Cycles3, Cycles2, Cycles8>,
    ws1_second_access: 7, // true = 4, false = 1
    ws2_first_access: 8-9 = WaitstateFirstAccess<Cycles4, Cycles3, Cycles2, Cycles8>,
    ws2_second_access: 10, // true = 8, false = 1
    phi_terminal_output: 11-12 = PhiTerminalOutput<Disabled, Freq4MHz, Freq8MHz, Freq16MHz>,
    game_pak_prefetch_buffer: 14,
    game_pak_is_cgb: 15,
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum WaitstateFirstAccess {
  Cycles4 = 0,
  Cycles3 = 1,
  Cycles2 = 2,
  Cycles8 = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum PhiTerminalOutput {
  Disabled = 0,
  Freq4MHz = 1,
  Freq8MHz = 2,
  Freq16MHz = 3,
}
