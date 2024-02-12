#![no_std]
#![no_main]

use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57200);

    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());

    let x = pins.a0.into_analog_input(&mut adc);
    let y = pins.a1.into_analog_input(&mut adc);
    let z = pins.a2.into_analog_input(&mut adc);

    loop {
        let (mut x_raw, mut y_raw, mut z_raw) = (0, 0, 0);
        for _ in 0..10 {
            x_raw += x.analog_read(&mut adc);
            y_raw += y.analog_read(&mut adc);
            z_raw += z.analog_read(&mut adc);
        }
        x_raw /= 10;
        y_raw /= 10;
        z_raw /= 10;
        ufmt::uwriteln!(
            &mut serial,
            "{}, {}, {}",
            map(x_raw),
            map(y_raw),
            map(z_raw)
        )
        .unwrap();
        arduino_hal::delay_ms(1000);
    }
}

fn map(val: u16) -> i16 {
    (val as i16 - 337) * 100 / 68
}
