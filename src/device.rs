use esp32c3_hal::clock::{ClockControl, Clocks};
use esp32c3_hal::peripherals::Peripherals;
use esp32c3_hal::prelude::*;

pub struct Device<'d> {
    pub peripherals: Peripherals,
    pub clocks: Clocks<'d>,
}

impl<'d> Device<'d> {
    pub fn new() -> Self {
        // We _must_ be the only one managing the peripherals. If it's guaranteed that we
        // are the only ones taking the peripherals, we can use `take()` but that's not a
        // guarantee, so we use `steal` instead.
        let peripherals = unsafe { Peripherals::steal() };
        let system = peripherals.SYSTEM.split();
        let clocks = ClockControl::max(system.clock_control).freeze();

        // We have to take peripherals again since, in order to split system, we have to
        // do a partial move of peripherals.SYSTEM.
        let peripherals = unsafe { Peripherals::steal() };

        Device {
            peripherals,
            clocks,
        }
    }
}
