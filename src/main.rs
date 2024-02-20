#![no_std]
#![no_main]
#![feature(ascii_char)]

use arduino_hal::spi::SerialClockRate;
use nrf24l01::NRF24L01;
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    ufmt::uwriteln!(&mut serial, "Hello from Arduino!\r").unwrap();

    let (spi, cs) = arduino_hal::Spi::new(
        dp.SPI,
        pins.d13.into_output(),        // CLOCK
        pins.d11.into_output(),        // MOSI
        pins.d12.into_pull_up_input(), // MISO
        pins.d10.into_output(),        // CS
        arduino_hal::spi::Settings {
            data_order: arduino_hal::spi::DataOrder::MostSignificantFirst,
            clock: SerialClockRate::OscfOver8,
            mode: nrf24l01::MODE,
        },
    );

    let ce = pins.d8.into_output();
    let mut radio = NRF24L01::new(spi, cs, ce, 1, 8).unwrap();

    #[cfg(feature = "sender")]
    radio.set_taddr(&b"serv1"[..]).unwrap();
    #[cfg(feature = "reciver")]
    radio.set_raddr(&b"serv1"[..]).unwrap();

    radio.config().unwrap();

    ufmt::uwriteln!(serial, "radio acquired, great success!").unwrap();
    loop {
        #[cfg(feature = "sender")]
        {
            if !radio.is_sending().unwrap() {
                radio.send(&b"hello world!"[..8]).unwrap();
            }
        }
        #[cfg(feature = "receiver")]
        {
            if radio.data_ready().unwrap() {
                let mut buffer = [0; 8];
                radio.get_data(&mut buffer).unwrap();
                let msg = unsafe { buffer.as_ascii_unchecked().as_str() };
                ufmt::uwriteln!(serial, "{}", msg).unwrap();
            }
        }
        arduino_hal::delay_ms(1000);
    }
}
