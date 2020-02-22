//! Timers

use gd32vf103_pac::CTIMER;

// This right now just gets the systick register, system timer, or mtime.
// I believe system timer is the correct name, as the documentation seems to
// imply mtimer is instruction count, while the system timer increments on
// clock pulses.
// todo: A more proper name? I may prefer core timer
// this module may cause software interrupt, thus may be controlled by multiple
// contexts (e.g. modification in interrupts), so I prefer design as ownership
// acquire module (CoreTimer is a new-type struct of CTIMER register block).
/// CTIMER
pub struct CoreTimer {
    ctimer: CTIMER
}

impl CoreTimer {
    pub fn new(ctimer: CTIMER) -> Self {
        CoreTimer { ctimer }
    }

    pub fn free(self) -> CTIMER {
        self.ctimer
    }

    pub fn get_value(&self) -> u64 {
        // Hi is systick1
        let hi: u32 = self.ctimer.mtime_hi.read().bits();
        let lo: u32 = self.ctimer.mtime_lo.read().bits();
        if hi == self.ctimer.mtime_hi.read().bits() {
            return (hi as u64) << 32 | (lo as u64);
        } else {
            return (self.ctimer.mtime_hi.read().bits() as u64) << 32
                | (self.ctimer.mtime_lo.read().bits() as u64);
        }
    }

    // This chip is a 32-bit MCU. Leave u32 functions here for convenience.
    #[inline]
    pub fn get_value_hi(&self) -> u32 {
        self.ctimer.mtime_hi.read().bits()
    }

    #[inline]
    pub fn get_value_lo(&self) -> u32 {
        self.ctimer.mtime_lo.read().bits()
    }

    // todo: more functions for mtimercmp, mstop and msip.
}
