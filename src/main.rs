#![no_main]
#![no_std]

extern crate panic_halt;

extern crate stm32f1xx_hal;

use cortex_m_rt::entry;
use cortex_m_semihosting::{hprintln};

#[entry]
fn main() -> ! {
    hprintln!("Hello, world!").unwrap();

    loop {}
}
