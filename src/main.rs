#![no_std]
#![no_main]

use lcd_lcm1602_i2c::Lcd;
use panic_halt as _;

const LCD_ADDRESS: u8 = 0x27;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 9600);
    let mut i2c = arduino_hal::I2c::new(
        dp.TWI,
        pins.a4.into_pull_up_input(),
        pins.a5.into_pull_up_input(),
        50000,
    );
    let mut delay = arduino_hal::Delay::new();
    let mut lcd = Lcd::new(&mut i2c, &mut delay)
        .address(LCD_ADDRESS)
        .cursor_on(false)
        .rows(2)
        .init()
        .unwrap();
    lcd.clear().unwrap();

    let led = &mut pins.d13.into_output();
    ufmt::uwriteln!(serial, "Ready").unwrap();

    loop {
        led.toggle();
        let byte = serial.read_byte();
        ufmt::uwrite!(lcd, "{}", byte as char).unwrap();
    }
}
