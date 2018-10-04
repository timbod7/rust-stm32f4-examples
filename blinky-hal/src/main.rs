#![no_main]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate panic_abort;

extern crate stm32f407g_disc as board;
extern crate embedded_hal as hal;

use cortex_m_rt::entry;

use board::hal::delay::Delay;
use board::hal::prelude::*;
use board::hal::stm32;
use board::gpio;
use board::gpio::gpiod::{PD12, PD13, PD14, PD15};


use hal::digital::OutputPin;

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

        // Configure LED outputs  
        let mut leds = Leds {
          green: gpiod.pd12.into_push_pull_output(),
          orange: gpiod.pd13.into_push_pull_output(),
          red: gpiod.pd14.into_push_pull_output(),
          blue: gpiod.pd15.into_push_pull_output(),
        };

        // Constrain clock registers
        let mut rcc = p.RCC.constrain();

        // Configure clock to 168 MHz (i.e. the maximum) and freeze it
        let clocks = rcc.cfgr.sysclk(168.mhz()).freeze();

        // Get delay provider
        let mut delay = Delay::new(cp.SYST, clocks);

        loop {
            // Turn LED on
            leds.blue.set_high();

            // Delay twice for half a second due to limited timer resolution
            delay.delay_ms(500_u16);
            delay.delay_ms(500_u16);

            // Turn LED off
            leds.blue.set_low();

            // Delay twice for half a second due to limited timer resolution
            delay.delay_ms(500_u16);
            delay.delay_ms(500_u16);
        }
    }

    loop {}
}