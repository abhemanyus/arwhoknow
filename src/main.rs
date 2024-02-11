#![no_std]
#![no_main]

use dht_sensor::dht11;
use embedded_hal::blocking::delay::DelayMs;
use panic_halt as _;

const LCD_ADDRESS: u8 = 0x27;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut i2c = arduino_hal::I2c::new(
        dp.TWI,
        pins.a4.into_pull_up_input(),
        pins.a5.into_pull_up_input(),
        50000,
    );
    let mut delay = arduino_hal::Delay::new();
    let mut lcd = lcd_lcm1602_i2c::Lcd::new(&mut i2c, &mut delay)
        .address(LCD_ADDRESS)
        .cursor_on(false) // no visible cursos
        .rows(2) // two rows
        .init()
        .unwrap();

    let mut d2 = pins.d2.into_opendrain_high();
    d2.set_high();
    /*
     * For examples (and inspiration), head to
     *
     *     https://github.com/Rahix/avr-hal/tree/main/examples
     *
     * NOTE: Not all examples were ported to all boards!  There is a good chance though, that code
     * for a different board can be adapted for yours.  The Arduino Uno currently has the most
     * examples available.
     */

    let mut led = pins.d13.into_output();

    let mut delay = arduino_hal::Delay::new();
    delay.delay_ms(2000_u16);
    loop {
        led.toggle();
        lcd.clear().unwrap();
        match dht11::read(&mut delay, &mut d2) {
            Ok(dht11::Reading {
                temperature,
                relative_humidity,
            }) => lcd.write_str("temp").unwrap(),
            Err(err) => match err {
                dht_sensor::DhtError::PinError(_) => lcd.write_str("pin err").unwrap(),
                dht_sensor::DhtError::ChecksumMismatch => lcd.write_str("checksum").unwrap(),
                dht_sensor::DhtError::Timeout => lcd.write_str("timeout").unwrap(),
            },
        }
        delay.delay_ms(1000_u16);
        arduino_hal::delay_ms(1000);
    }
}
