//! Prints "Hello, world!" on the OpenOCD console using semihosting
//!
//! ---

#![no_main]
#![no_std]

#[macro_use]
extern crate cortex_m_rt as rt;

extern crate cortex_m;
extern crate cast;
extern crate stm32f4;
extern crate panic_semihosting;

use cast::{u16, u32};
use rt::ExceptionFrame;
use stm32f4::stm32f407;

mod frequency {
    /// Frequency of APB1 bus (TIM6 is connected to this bus)
    pub const APB1: u32 = 8_000_000;
}

/// Timer frequency
const FREQUENCY: u32 = 1;

entry!(main);

const U16_MAX: u16 = 0xffff;

// On STM32F407G-DISC1 board, user LEDS are
//   LD3: orange @ PD13
//   LD4: green  @ PD12
//   LD5: red    @ PD14
//   LD6: blue   @ PD15

fn main() -> ! {
    let peripherals = stm32f407::Peripherals::take().unwrap();

    // Critical section, this closure is non-preemptable
    cortex_m::interrupt::free(|_cs| {

        // INITIALIZATION PHASE

        // Power up the relevant peripherals
        peripherals.RCC.ahb1enr.write(|w| w.gpioden().set_bit());
        peripherals.RCC.apb1enr.write(| w| w.tim6en().set_bit());

        // Configure the pin PD12 as a pullup output pin
        peripherals.GPIOD.otyper.write(|w| w.ot12().clear_bit());
        peripherals.GPIOD.moder.write(|w| w.moder12().output());
        peripherals.GPIOD.pupdr.write(|w| w.pupdr12().pull_up());

        // Configure TIM6 for periodic timeouts
        let ratio = frequency::APB1 / FREQUENCY;
        let psc = u16((ratio - 1) / u32(U16_MAX)).unwrap();
        let arr = u16(ratio / u32(psc + 1)).unwrap();
        unsafe {
            peripherals.TIM6.psc.write(|w| w.psc().bits(psc));
            peripherals.TIM6.arr.write(|w| w.arr().bits(arr));
        };
        peripherals.TIM6.cr1.write(|w| w.opm().clear_bit());

        // Start the timer
        peripherals.TIM6.cr1.modify(|_, w| w.cen().set_bit());

        // APPLICATION LOGIC
        let mut state = false;
        loop {
            // Wait for an update event
            while peripherals.TIM6.sr.read().uif().bit_is_clear() {}

            // Clear the update event flag
            peripherals.TIM6.sr.modify(|_, w| w.uif().clear_bit());

            // Toggle the state
            state = !state;

            // Blink the LED
            peripherals.GPIOD.odr.write(|w| w.odr12().bit(state));
        }
    })
}

exception!(HardFault, hard_fault);

fn hard_fault(ef: &ExceptionFrame) -> ! {
    panic!("HardFault at {:#?}", ef);
}

exception!(*, default_handler);

fn default_handler(irqn: i16) {
    panic!("Unhandled exception (IRQn = {})", irqn);
}
