//! An application with one task
#![deny(unsafe_code)]
#![deny(warnings)]
#![no_std]
#![no_main]

extern crate cortex_m;
#[macro_use(entry)]
extern crate cortex_m_rt as rt;
extern crate cortex_m_rtfm as rtfm;
extern crate panic_abort;
extern crate stm32f4;

use rtfm::{app, Threshold};
use stm32f4::stm32f407;
use stm32f4::stm32f407::GPIOD;

app! {
    device: stm32f407,

    // Here data resources are declared
    //
    // Data resources are static variables that are safe to share across tasks
    resources: {
        // Declaration of resources looks exactly like declaration of static
        // variables
        static ON: bool = false;
    },

    // Here tasks are declared
    //
    // Each task corresponds to an interrupt or an exception. Every time the
    // interrupt or exception becomes *pending* the corresponding task handler
    // will be executed.
    tasks: {
        // Here we declare that we'll use the SysTick exception as a task
        SysTick: {
            // Path to the task handler
            path: sys_tick,

            // These are the resources this task has access to.
            //
            // The resources listed here must also appear in `app.resources`
            resources: [ON],
        },
    }
}

fn init(p: init::Peripherals, _r: init::Resources) {

    // Power up the relevant peripherals
    p.device.RCC.ahb1enr.write(|w| w.gpioden().set_bit());
    p.device.RCC.apb1enr.write(| w| w.tim6en().set_bit());

    // Configure the pin PD12 as a pullup output pin
    p.device.GPIOD.otyper.write(|w| w.ot12().clear_bit());
    p.device.GPIOD.moder.write(|w| w.moder12().output());
    p.device.GPIOD.pupdr.write(|w| w.pupdr12().pull_up());
}

fn idle() -> ! {
    loop {
        rtfm::wfi();
    }
}

// This is the task handler of the SysTick exception
//
// `_t` is the preemption threshold token. We won't use it in this program.
//
// `r` is the set of resources this task has access to. `SysTick::Resources`
// has one field per resource declared in `app!`.
#[allow(unsafe_code)]
fn sys_tick(_t: &mut Threshold, mut r: SysTick::Resources) {
    // toggle state
    *r.ON = !*r.ON;

    unsafe {
      (*GPIOD::ptr()).odr.write(|w| w.odr12().bit(*r.ON));
    }
}
