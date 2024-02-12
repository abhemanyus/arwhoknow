#![no_std]
#![no_main]

use dht_sensor::{dht11, DhtReading};
use lcd_lcm1602_i2c::Lcd;
use panic_halt as _;

const LCD_ADDRESS: u8 = 0x27;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57200);

    let mut i2c = arduino_hal::I2c::new(
        dp.TWI,
        pins.a4.into_pull_up_input(),
        pins.a5.into_pull_up_input(),
        50000,
    );
    arduino_hal::delay_ms(2000);
    let mut delay = arduino_hal::Delay::new();
    let mut lcd = Lcd::new(&mut i2c, &mut delay)
        .address(LCD_ADDRESS)
        .cursor_on(false)
        .rows(2)
        .init()
        .unwrap();
    lcd.clear().unwrap();

    let mut pin2 = pins.d2.into_opendrain_high();
    let mut delay = arduino_hal::Delay::new();

    ufmt::uwriteln!(serial, "{}", "waiting for sensor...").unwrap();
    ufmt::uwrite!(lcd, "{}", "waiting for sensor...").unwrap();
    arduino_hal::delay_ms(2000);

    loop {
        lcd.clear().unwrap();
        match dht11::Reading::read(&mut delay, &mut pin2) {
            Ok(dht11::Reading {
                temperature,
                relative_humidity,
            }) => {
                ufmt::uwriteln!(serial, "{}c, {}%", temperature, relative_humidity).unwrap();
                ufmt::uwrite!(lcd, "{}c, {}%", temperature, relative_humidity).unwrap();
            }
            Err(_e) => ufmt::uwriteln!(serial, "Error {}", "Unable to read").unwrap(),
        }
        arduino_hal::delay_ms(2000);
    }
}
