#![no_main]
#![no_std]

extern crate panic_halt;

use cortex_m_rt::entry;
use stm32f1xx_hal::{
    prelude::*,
    stm32,
    i2c::{
        BlockingI2c,
        Mode
    },
    delay::Delay
};
use bno055::{
    Bno055,
    BNO055OperationMode,
    Quaternion,
    Error
};
use cortex_m_semihosting::{hprintln};


#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let mut rcc = dp.RCC.constrain();
    let mut flash = dp.FLASH.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);
    let mut afio = dp.AFIO.constrain(&mut rcc.apb2);
    
    let scl = gpiob.pb8.into_alternate_open_drain(&mut gpiob.crh);
    let sda = gpiob.pb9.into_alternate_open_drain(&mut gpiob.crh);
    
    let i2c = BlockingI2c::i2c1(
        dp.I2C1,
        (scl, sda),
        &mut afio.mapr,
        Mode::Standard {
            frequency: 100_000,
        },
        clocks,
        &mut rcc.apb1,
        1000,
        10,
        1000,
        1000,
    );

    let delay = Delay::new(cp.SYST, clocks);

    let mut imu = Bno055::new(i2c, delay);

    match imu.init() {
        Ok(_)       => hprintln!("worked").unwrap(),
        Err(err)    => hprintln!("didnt worked {:?}", err).unwrap()
    }

    imu.set_mode(BNO055OperationMode::NDOF).unwrap();

    loop {
        let quat: Quaternion<f32> = imu.quaternion().unwrap();
        hprintln!("quat: {}", quat).unwrap();
    }
}
