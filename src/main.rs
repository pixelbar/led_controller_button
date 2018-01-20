//! Prints "Hello, world!" on the OpenOCD console using semihosting
//!
//! ---

#![feature(used, const_size_of)]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate cortex_m_semihosting;
extern crate peripherals;

use cortex_m::asm;

fn configure(rcc: &peripherals::RCC, gpioa: &peripherals::GPIOA){
    // TODO: Enable the USB port
    // Enable the 2 IO strips (A and B)
    rcc.apb2enr.modify(|_, w| 
        w.iopaen().enabled()
    );

    // Configure the A pins to be output + push
    gpioa.crl.modify(|_, w| 
        w.mode0().input().cnf0().open()
         .mode1().output().cnf1().push()
    );
}


fn make_go_faster(rcc: &peripherals::RCC, flash: &peripherals::FLASH) {
    rcc.cr.modify(|_, w| w.hseon().enabled());
    while !rcc.cr.read().hserdy().is_ready() {}
    flash.acr.modify(|_, w| w.prftbe().enabled());
    flash.acr.modify(|_, w| w.latency().two());
    rcc.cfgr.modify(|_, w| w
                    .hpre().div1()
                    .ppre2().div1()
                    .ppre1().div2()
                    // .adcpre().bits(8)
                    .pllsrc().external()
                    .pllxtpre().div1()
                    .pllmul().mul9()
    );
    rcc.cr.modify(|_, w| w.pllon().enabled());
    while rcc.cr.read().pllrdy().is_unlocked() {}
    rcc.cfgr.modify(|_,w| w.sw().pll());
    while !rcc.cfgr.read().sws().is_pll() {}
}

fn main() {
    let rcc = unsafe { &*peripherals::RCC.get() };
    let flash = unsafe { &*peripherals::FLASH.get() };
    let gpioa = unsafe { &*peripherals::GPIOA.get() };

    configure(&rcc, &gpioa);
    make_go_faster(&rcc, &flash);

    let mut was_set = false;
    let mut output_high = false;

    loop {
        let idr = gpioa.idr.read();
        if idr.idr0().bit_is_set() {
            if was_set {
                continue;
            }
            was_set = true;

            output_high = !output_high;
            if output_high {
                gpioa.bsrr.write(|w| 
                    w.bs1().set_bit()
                );
            } else {
                gpioa.bsrr.write(|w|
                    w.br1().set_bit()
                );
            }

            for _ in 0..10000000 {
                asm::nop();
            }

        } else {
            was_set = false;
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
