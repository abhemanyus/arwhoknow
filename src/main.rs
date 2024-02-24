#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use core::sync::atomic::{AtomicBool, Ordering};

use arduino_hal::pac::tc1::tccr1b::CS1_A;
use embedded_hal::{
    serial::Read,
    timer::{CountDown, Periodic},
};
use panic_halt as _;

static SAMPLE_NOW: AtomicBool = AtomicBool::new(false);

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    let tx = pins.d8.into_output();
    let rx = pins.d7.into_floating_input();

    let tmr = Timer::new(dp.TC1, 4800);

    let mut software_serial = bitbang_hal::serial::Serial::new(tx, rx, tmr);

    unsafe {
        avr_device::interrupt::enable();
    }

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
        while !SAMPLE_NOW.load(Ordering::SeqCst) {}
        SAMPLE_NOW.store(false, Ordering::SeqCst);
        Ok(())
    }
}

impl Periodic for Timer {}

struct Timer {
    timer: arduino_hal::pac::TC1,
}

impl Timer {
    fn new(timer: arduino_hal::pac::TC1, target_hz: u32) -> Self {
        use arduino_hal::clock::Clock;
        const ARDUINO_UNO_CLOCK_FREQUENCY_HZ: u32 = arduino_hal::DefaultClock::FREQ;
        const CLOCK_SOURCE: CS1_A = CS1_A::DIRECT;
        let clock_divisor: u32 = match CLOCK_SOURCE {
            CS1_A::DIRECT => 1,
            CS1_A::PRESCALE_8 => 8,
            CS1_A::PRESCALE_64 => 64,
            CS1_A::PRESCALE_256 => 256,
            CS1_A::PRESCALE_1024 => 1024,
            CS1_A::NO_CLOCK | CS1_A::EXT_FALLING | CS1_A::EXT_RISING => 1,
        };
        let ticks = calc_overflow(ARDUINO_UNO_CLOCK_FREQUENCY_HZ, target_hz, clock_divisor) as u16;
        timer.tccr1a.write(|w| w.wgm1().bits(0b00));
        timer.tccr1b.write(|w| {
            w.cs1()
                //.prescale_256()
                .variant(CLOCK_SOURCE)
                .wgm1()
                .bits(0b01)
        });
        timer.ocr1a.write(|w| w.bits(ticks));
        timer.timsk1.write(|w| w.ocie1a().set_bit());
        Self { timer }
    }
}

#[avr_device::interrupt(atmega328p)]
fn TIMER1_COMPA() {
    SAMPLE_NOW.store(true, Ordering::SeqCst);
}
const fn calc_overflow(clock_hz: u32, target_hz: u32, prescale: u32) -> u32 {
    clock_hz / target_hz / prescale - 1
}
