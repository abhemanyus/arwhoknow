#![no_std]
#![no_main]

use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    ufmt::uwriteln!(&mut serial, "Hello from Arduino!\r").unwrap();

    let led = pins.a0.into_pull_up_input();

    loop {
        ufmt::uwriteln!(serial, "{}", 0).unwrap();
        arduino_hal::delay_ms(500);
    }
}
