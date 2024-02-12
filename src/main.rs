#![no_std]
#![no_main]

use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 38400);

    let led = &mut pins.d13.into_output();

    loop {
        match serial.read_byte() {
            1u8 => {
                led.set_low();
                ufmt::uwrite!(serial, "LED: OFF").unwrap();
            }
            2u8 => {
                led.set_high();
                arduino_hal::delay_ms(2000);
                led.set_low();
                ufmt::uwrite!(serial, "LED: ON").unwrap();
            }
            d => {
                ufmt::uwrite!(serial, "{}", d).unwrap();
                led.set_high();
                arduino_hal::delay_ms(250);
                led.set_low();
                arduino_hal::delay_ms(250);
                led.set_high();
                arduino_hal::delay_ms(250);
                led.set_low();
            }
        }
        arduino_hal::delay_ms(1000);
    }
}
