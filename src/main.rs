#![no_std]
#![no_main]

use panic_halt as _;

const LCD_ADDRESS: u8 = 0x27;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut i2c = arduino_hal::I2c::new(dp.TWI, pins.a4.into_pull_up_input(), pins.a5.into_pull_up_input(), 50000);
    let mut delay = arduino_hal::Delay::new();
    let mut lcd = liquidcrystal_i2c_rs::Lcd::new(&mut i2c, LCD_ADDRESS, &mut delay).unwrap();
    lcd.set_display(liquidcrystal_i2c_rs::Display::On).unwrap();
    lcd.set_backlight(liquidcrystal_i2c_rs::Backlight::Off).unwrap();
    lcd.print("Peter piper pick").unwrap();
    lcd.print("ed a peck of").unwrap();

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

    loop {
        led.toggle();
        arduino_hal::delay_ms(1000);
    }
}
