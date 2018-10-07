#![no_main]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate panic_itm;

extern crate stm32f407g_disc as board;
extern crate embedded_hal as hal;

use cortex_m_rt::entry;

use board::hal::delay::Delay;
use board::hal::prelude::*;
use board::hal::stm32;
use board::gpio;
use board::gpio::gpiod::{PD12, PD13, PD14, PD15};
use board::hal::serial;
use board::hal::serial::{Serial};

#[macro_use(block)]
extern crate nb;
mod sbus;

use hal::digital::OutputPin;

use cortex_m::iprintln;
use cortex_m::peripheral::Peripherals;

struct Leds {
   green:  PD12<gpio::Output<gpio::PushPull>>,
   orange: PD13<gpio::Output<gpio::PushPull>>,
   red:    PD14<gpio::Output<gpio::PushPull>>,
   blue:   PD15<gpio::Output<gpio::PushPull>>,
}

#[entry]
fn main() -> ! {
    if let (Some(p), Some(cp)) = (stm32::Peripherals::take(), Peripherals::take()) {
        let gpiod = p.GPIOD.split();
        let mut itm = cp.ITM;

        // Constrain clock registers
        let mut rcc = p.RCC.constrain();

        // Configure clock to 168 MHz (i.e. the maximum) and freeze it
        let clocks = rcc.cfgr.sysclk(168.mhz()).freeze();

        // USART2 at PD5 (TX) and PD6(RX)
        let txpin = gpiod.pd5.into_alternate_af7();
        let rxpin = gpiod.pd6.into_alternate_af7();
        let config = serial::config::Config::default()
            .baudrate(100_000.bps())
            .parity_even()
            .wordlength_9()
            .stopbits(serial::config::StopBits::STOP2);
        let serial = Serial::usart2(p.USART2, (txpin, rxpin), config, clocks).unwrap();

        let (mut tx, mut rx) = serial.split();

        // Get delay provider
        let mut delay = Delay::new(cp.SYST, clocks);

        iprintln!(&mut itm.stim[0], "start" );

        let mut state = sbus::SbusReadState::default();

        loop {
            let received = block!(rx.read());
            match received {
                Ok(c) => {
                    let complete = sbus::process_char(&mut state, c);
                    if complete {
                        iprintln!(&mut itm.stim[0], "{} {}", state.frame.channels[0], state.frame.channels[1] );
                    }
                },
                Err(e) => {
                    iprintln!(&mut itm.stim[0], "err" );
                   
                },
            }

        }
    }

    loop {}
}