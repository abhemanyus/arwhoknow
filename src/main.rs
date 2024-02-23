#![no_std]
#![no_main]

use arduino_hal::{clock::MHz16, hal::delay::Delay};
use embedded_hal::{
    blocking::delay::DelayUs,
    serial::Read,
    timer::{CountDown, Periodic},
};
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    let tx = pins.d8.into_output();
    let rx = pins.d7.into_floating_input();

    let tmr = Timer {
        timer: Delay::new(),
    };

    let mut software_serial = bitbang_hal::serial::Serial::new(tx, rx, tmr);

    loop {
        // ufmt::uwriteln!(serial, "hello!").unwrap();
        let word = software_serial.read().unwrap();
        serial.write_byte(word);
        // tmr.wait().unwrap();
    }
}

impl CountDown for Timer {
    type Time = u16;

    fn start<T>(&mut self, _count: T)
    where
        T: Into<Self::Time>,
    {
    }

    fn wait(&mut self) -> nb::Result<(), void::Void> {
        self.timer.delay_us(104u16);
        Ok(())
    }
}

impl Periodic for Timer {}

struct Timer {
    timer: Delay<MHz16>,
}
