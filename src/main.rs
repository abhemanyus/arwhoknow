#![no_std]
#![no_main]

use core::convert::Infallible;

use arduino_hal::{
    hal::port::{PB0, PB1, PD2, PD3, PD4, PD5, PD6, PD7},
    port::{
        mode::{Input, OpenDrain, PullUp},
        Pin,
    },
};
use embedded_hal::digital::v2::InputPin;
use keypad::{keypad_new, keypad_struct};
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    let keypad = keypad_new!(Keypad {
        rows: (
            pins.d9.into_pull_up_input(),
            pins.d8.into_pull_up_input(),
            pins.d7.into_pull_up_input(),
            pins.d6.into_pull_up_input()
        ),
        columns: (
            pins.d5.into_opendrain(),
            pins.d4.into_opendrain(),
            pins.d3.into_opendrain(),
            pins.d2.into_opendrain(),
        ),
    });

    let keys = keypad.decompose();

    ufmt::uwriteln!(&mut serial, "Hello from Arduino!").unwrap();

    let mut last_key = 17;
    let mut pressed = true;
    loop {
        pressed = false;
        for (index, key) in keys.iter().flatten().enumerate() {
            if key.is_low().unwrap() {
                pressed = true;
                if last_key != index {
                    ufmt::uwriteln!(serial, "{}", KEY_MAP[index]).unwrap();
                }
                last_key = index;
            }
        }
        if pressed == false {
            last_key = 17;
        }
        arduino_hal::delay_ms(100);
    }
}

const KEY_MAP: [char; 16] = [
    '1', '2', '3', 'A', '7', '8', '9', 'C', '4', '5', '6', 'B', '*', '0', '#', 'D',
];

keypad_struct! {
    pub struct Keypad<Error = Infallible> {
        rows: (
            Pin<Input<PullUp>, PB1>,
            Pin<Input<PullUp>, PB0>,
            Pin<Input<PullUp>, PD7>,
            Pin<Input<PullUp>, PD6>,
        ),
        columns: (
            Pin<OpenDrain, PD5>,
            Pin<OpenDrain, PD4>,
            Pin<OpenDrain, PD3>,
            Pin<OpenDrain, PD2>,
        ),
    }
}
