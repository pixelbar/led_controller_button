//! Prints "Hello, world!" on the OpenOCD console using semihosting
//!
//! ---

#![feature(used)]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate cortex_m_semihosting;
extern crate peripherals;

use core::fmt::Write;

use cortex_m::asm;
use cortex_m_semihosting::hio;
use core::ops::Deref;

struct Frame {
    pub side_a: u32,
    pub side_b: u32,
}

fn configure(){
    let rcc: &peripherals::RCC = unsafe { &*peripherals::RCC.get() };
    // Enable the USB port
    // rcc.apb1enr.modify(|_, w| 
    //     w.usben().enabled()
    // );
    // Enable the 2 IO strips (A and B)
    rcc.apb2enr.modify(|_, w| 
        w.iopaen().enabled()
         .iopben().enabled()
    );

    // Get a reference to the 2 IO strips and the USB module
    let gpioa = unsafe { &*peripherals::GPIOA.get() };
    let gpiob = unsafe { &*peripherals::GPIOB.get() };
    // let usb = peripherals::USB.borrow(cs).deref();

    // Configure the A pins to be output + push
    gpioa.crl.modify(|_, w| 
        w.mode0().output().cnf0().push()
         .mode1().output().cnf1().push()
         .mode2().output().cnf2().push()
         .mode3().output().cnf3().push()
         .mode4().output().cnf4().push()
         .mode5().output().cnf5().push()
         .mode6().output().cnf6().push()
         .mode7().output().cnf7().push()
    );
        
    // Configure the B pins to be output + push
    gpiob.crh.modify(|_, w| 
        w.mode8().output().cnf8().push()
         .mode9().output().cnf9().push()
         .mode10().output().cnf10().push()
         .mode11().output().cnf11().push()
         .mode12().output().cnf12().push()
         .mode13().output().cnf13().push()
         .mode14().output().cnf14().push()
         .mode15().output().cnf15().push()
    );
}

fn render(frames: &[Frame;100], count: usize) {
    let gpioa = unsafe { &*peripherals::GPIOA.get() };
    let gpiob = unsafe { &*peripherals::GPIOB.get() };
    for _ in 0..count {
        for frame in frames.iter() {
            gpioa.bsrr.write(|w| unsafe {
                w.bits(frame.side_a)
            });
            gpiob.bsrr.write(|w| unsafe {
                w.bits(frame.side_b)
            });
            /* #[cfg(not(debug_assertions))]
            for _ in 0..100 {
                asm::nop();
            } */
        }
    }
}

const SET_ALL_LIGHTS: u32 = 0b00000000_00000000_00000000_00001111;
const CLEAR_ALL_LIGHTS: u32 = 0b00000000_00001111_00000000_00000000;

fn main() {
    let mut frames: [Frame;100] = unsafe { ::core::mem::zeroed() };
    frames[0].side_a = SET_ALL_LIGHTS;

    let mut frame_read_pointer = 0;

    configure();

    loop {
        for index in 1..100 {
            frames[index].side_a = CLEAR_ALL_LIGHTS;
            render(&frames, 100);
            frames[index].side_a = 0;
        }
        for index in 1..100 {
            frames[100-index].side_a = CLEAR_ALL_LIGHTS;
            render(&frames, 100);
            frames[100-index].side_a = 0;
        }
    }
}

// As we are not using interrupts, we just register a dummy catch all handler
#[link_section = ".vector_table.interrupts"]
#[used]
static INTERRUPTS: [extern "C" fn(); 240] = [default_handler; 240];

extern "C" fn default_handler() {
    asm::bkpt();
}
