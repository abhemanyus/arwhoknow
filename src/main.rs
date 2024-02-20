#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use arduino_hal::spi::SerialClockRate;
use nrf24l01::NRF24L01;
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    ufmt::uwriteln!(&mut serial, "Hello from Arduino!\r").unwrap();

    let (spi, cns) = arduino_hal::Spi::new(
        dp.SPI,
        pins.d13.into_output(),
        pins.d11.into_output(),
        pins.d12.into_pull_up_input(),
        pins.d10.into_output(),
        arduino_hal::spi::Settings {
            data_order: arduino_hal::spi::DataOrder::MostSignificantFirst,
            clock: SerialClockRate::OscfOver8,
            mode: nrf24l01::MODE,
        },
    );

    let ce = pins.d8.into_output();
    let mut radio = NRF24L01::new(spi, cns, ce, 1, 4).unwrap();

    radio.set_raddr("serv1".as_bytes()).unwrap();
    radio.set_taddr("clie1".as_bytes()).unwrap();

    radio.config().unwrap();

    ufmt::uwriteln!(serial, "radio acquired, great success!").unwrap();
    loop {
        #[cfg(feature = "sender")]
        {
            ufmt::uwriteln!(serial, "sending...").unwrap();
            if !radio.is_sending().unwrap() {
                radio.send(&"hello world!".as_bytes()[..8]).unwrap();
            }
            arduino_hal::delay_ms(1000);
        }
        #[cfg(feature = "receiver")]
        {
            if radio.data_ready().unwrap() {
                ufmt::uwriteln!(serial, "receiving...").unwrap();
                let mut buffer = [0; 8];
                radio.get_data(&mut buffer).unwrap();
                for c in buffer {
                    ufmt::uwrite!(serial, "{} ", c).unwrap();
                }
            }
        }
        ufmt::uwriteln!(serial, "").unwrap();
    }
}
