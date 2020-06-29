// An RTFM application with a single task that is scheduled repeatedly to toggle an LED
#![deny(unsafe_code)]
#![deny(warnings)]
#![no_std]
#![no_main]

extern crate rtfm;
extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate panic_abort;
extern crate stm32f4;

extern crate stm32f407g_disc as board; // <-- board support crate (BSP). Depending on your board, you might have to choose another one
use board::{
    gpio::{
        Output, PushPull,
        gpiod::PD12,
    },
    hal::prelude::*,
};

use rtfm::cyccnt::{Instant, U32Ext};

const CORE_CLOCK_FREQUENCY: u32 = 168000000;

#[rtfm::app(device = board, monotonic = rtfm::cyccnt::CYCCNT, peripherals = true)]
const APP: () = {
    // static resources
    struct Resources {

        // pre-initialized variable
        #[init(false)] // <-- initial value
        on: bool,

        #[init(1)]
        toggle_interval_seconds: u32,

        // "late resource", initialized at runtime during init()
        // and returned in init::LateResources struct
        led_pin: PD12<Output<PushPull>>,
    }

    #[init(schedule = [toggle])]
    fn init(mut ctx: init::Context) -> init::LateResources {
        ctx.core.DWT.enable_cycle_counter();
        ctx.core.DCB.enable_trace(); // needed so the DWT cycle counter doesn't get disabled when no debugger is connected

        let rcc = ctx.device.RCC.constrain();
        rcc.cfgr.sysclk(CORE_CLOCK_FREQUENCY.hz()).freeze();

        // Configure the pin PD12 as a push-pull output pin
        let gpiod = ctx.device.GPIOD.split();
        let led_pin = gpiod.pd12.into_push_pull_output();

        // schedule the LED toggle task for immediate execution
        ctx.schedule.toggle(Instant::now()).unwrap();

        // late resources
        init::LateResources {
            led_pin: led_pin
        }
    }

    // this task requires access to led_pin to toggle the LED and
    // re-schedules itself toggle_interval_seconds later
    #[task(resources = [led_pin, toggle_interval_seconds], schedule = [toggle])]
    fn toggle(ctx: toggle::Context) {
        // toggle state
        ctx.resources.led_pin.toggle().unwrap();

        // re-schedule toggle()
        let execution_time = (CORE_CLOCK_FREQUENCY * (*ctx.resources.toggle_interval_seconds)).cycles();
        ctx.schedule.toggle(ctx.scheduled + execution_time).unwrap()
    }

    #[idle]
    fn idle(_ctx: idle::Context) -> ! {
        loop {}
    }

    // tasks override interrupt handlers. You'll need to list as
    // many interrupt handlers here as your application has tasks.
    extern "C" {
        fn USART1();
    }
};
