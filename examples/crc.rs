#![deny(warnings)]
#![deny(unsafe_code)]
#![no_main]
#![no_std]

//use cortex_m;
use cortex_m_rt as rt;
#[allow(unused_imports)]
//extern crate panic_halt;
use panic_semihosting;
use stm32g4xx_hal as hal;

use cortex_m_semihosting::hprintln;
use crate::hal::crc::*;
use crate::hal::prelude::*;
use crate::hal::stm32;
use rt::entry;

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().expect("cannot take peripherals");
    let mut rcc = dp.RCC.constrain();

    let mut crc = dp.CRC.constrain(&mut rcc);
    crc.reverse_input(InputReverse::Word);
    crc.reverse_output(true);

    loop {
        crc.reset();
        let hash_sum = crc.digest("123456789");
        hprintln!(
            "crc32: 0x{:x}, crc32b: 0x{:x}",
            hash_sum,
            hash_sum ^ 0xffffffff
        )
        .unwrap();

        crc.reset();
        let hash_sum = crc.digest("The quick brown fox jumps over the lazy dog");
        hprintln!(
            "crc32: 0x{:x}, crc32b: 0x{:x}",
            hash_sum,
            hash_sum ^ 0xffffffff
        )
        .unwrap();
    }
}
