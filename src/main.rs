#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use core::mem;

use arduino_hal::port::{mode::Output, Pin};
use avr_device::atmega328p::{tc1::tccr1b::CS1_A, TC1};
use panic_halt as _;
use ufmt::uwriteln;

struct InterruptState {
    blinker: Pin<Output>,
}

static mut INTERRUPT_STATE: mem::MaybeUninit<InterruptState> = mem::MaybeUninit::uninit();

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    ufmt::uwriteln!(&mut serial, "Hello from Arduino!\r").unwrap();

    /*
     * For examples (and inspiration), head to
     *
     *     https://github.com/Rahix/avr-hal/tree/main/examples
     *
     * NOTE: Not all examples were ported to all boards!  There is a good chance though, that code
     * for a different board can be adapted for yours.  The Arduino Uno currently has the most
     * examples available.
     */

    let led = pins.d13.into_output();

    unsafe {
        INTERRUPT_STATE = mem::MaybeUninit::new(InterruptState {
            blinker: led.downgrade(),
        });
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }

    let tmr1: TC1 = dp.TC1;
    rig_timer(&tmr1, &mut serial);

    unsafe {
        avr_device::interrupt::enable();
    }

    ufmt::uwriteln!(
        &mut serial,
        "configured timer output compare register = {}",
        tmr1.ocr1a.read().bits()
    )
    .unwrap();

    loop {
        avr_device::asm::sleep()
    }
}

pub const fn calc_overflow(clock_hz: u32, target_hz: u32, prescale: u32) -> u32 {
    clock_hz / target_hz / prescale - 1
}

pub fn rig_timer<W: ufmt::uWrite<Error = void::Void>>(tmr1: &TC1, serial: &mut W) {
    use arduino_hal::clock::Clock;
    const ARDUINO_UNO_CLOCK_FREQUENCY_HZ: u32 = arduino_hal::DefaultClock::FREQ;
    const CLOCK_SOURCE: CS1_A = CS1_A::PRESCALE_256;
    let clock_divisor: u32 = match CLOCK_SOURCE {
        CS1_A::DIRECT => 1,
        CS1_A::PRESCALE_8 => 8,
        CS1_A::PRESCALE_64 => 64,
        CS1_A::PRESCALE_256 => 256,
        CS1_A::PRESCALE_1024 => 1024,
        CS1_A::NO_CLOCK | CS1_A::EXT_FALLING | CS1_A::EXT_RISING => {
            uwriteln!(serial, "uhoh, code tried to set the clock source to something other than a static prescaler {}", CLOCK_SOURCE as usize).unwrap();
            1
        }
    };

    let ticks = calc_overflow(ARDUINO_UNO_CLOCK_FREQUENCY_HZ, 4, clock_divisor) as u16;
    ufmt::uwriteln!(
        serial,
        "configuring timer output compare register = {}",
        ticks
    )
    .unwrap();

    tmr1.tccr1a.write(|w| w.wgm1().bits(ticks as u8));
    tmr1.tccr1b
        .write(|w| w.cs1().variant(CLOCK_SOURCE).wgm1().bits(0b01));
    tmr1.ocr1a.write(|w| w.bits(ticks));
    tmr1.timsk1.write(|w| w.ocie1a().set_bit());
}

#[avr_device::interrupt(atmega328p)]
fn TIMER1_COMPA() {
    let state = unsafe { &mut *INTERRUPT_STATE.as_mut_ptr() };

    state.blinker.toggle();
}
