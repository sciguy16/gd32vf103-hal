//! Delay
use embedded_hal::blocking::delay::{DelayMs};
use crate::rcu::Clocks;
use crate::systick::Systick;
use crate::time::*;
use gd32vf103_pac::CTIMER;
/// Hardware timers
pub struct Delay{
    ctimer: CTIMER,
    clock_frequency : Hertz,
}


impl Delay {
    pub fn new(clocks: Clocks, ctimer: CTIMER) -> Self 
    {
        Delay {
            ctimer: ctimer,
            clock_frequency: clocks.ck_ahb(),
        }
    }
}

impl <T: Into<u32>> DelayMs<T> for Delay {
    // This doesn't wait for a systick tick, so may be off by a few ns. Sorry
    // The divide by two may be incorrect for other dividors. It should be 8
    // according to the clock diagram, but 2 is accurate. I suspect
    // this will need to change with further documentation updates.
    fn delay_ms(&mut self, ms: T) {
        let count : u32= (ms.into() * self.clock_frequency.0 / 1000 / (2));
        let tmp : u64 = Systick::get_systick(&self.ctimer);
        let end = tmp + count as u64;
        while Systick::get_systick(&self.ctimer) < end{
        }
    }
}