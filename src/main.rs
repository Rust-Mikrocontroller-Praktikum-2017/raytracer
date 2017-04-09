#![allow(dead_code)]
#![feature(collections, core_intrinsics)]

#![no_std]
#![no_main]

extern crate stm32f7_discovery as stm32f7;

// initialization routines for .data and .bss
extern crate r0;
extern crate collections;

use stm32f7::{system_clock, sdram, lcd, board, embedded};

mod vector;
mod math;
mod camera;
mod cameras;
mod scene;
mod scenes;
mod render;
mod intersection;
mod intersectables;
mod ray;
mod reflectionmodel;
mod display;
mod displays;
mod random;
mod texture;
mod textures;

use vector::Vec3;
use render::render;
use camera::Film;
// use cameras::perspective::PerspectiveCamera;
use cameras::orthographic::OrthographicCamera;
use scenes::spheres::SCENE_SPHERE;
use displays::stm32f7::Lcd as LcdDisplay;

#[no_mangle]
pub unsafe extern "C" fn reset() -> ! {
    extern "C" {
        static __DATA_LOAD: u32;
        static __DATA_END: u32;
        static mut __DATA_START: u32;
        static mut __BSS_START: u32;
        static mut __BSS_END: u32;
    }
    let data_load = &__DATA_LOAD;
    let data_start = &mut __DATA_START;
    let data_end = &__DATA_END;
    let bss_start = &mut __BSS_START;
    let bss_end = &__BSS_END;

    // initializes the .data section
    //(copy the data segment initializers from flash to RAM)
    r0::init_data(data_start, data_end, data_load);
    // zeroes the .bss section
    r0::zero_bss(bss_start, bss_end);

    // enable floating point unit
    let scb = stm32f7::cortex_m::peripheral::scb_mut();
    scb.cpacr.modify(|v| v | 0b1111 << 20);

    main(board::hw());
}

fn main(hw: board::Hardware) -> ! {
    let board::Hardware { rcc,
                          pwr,
                          flash,
                          fmc,
                          ltdc,
                          gpio_a,
                          gpio_b,
                          gpio_c,
                          gpio_d,
                          gpio_e,
                          gpio_f,
                          gpio_g,
                          gpio_h,
                          gpio_i,
                          gpio_j,
                          gpio_k,
                          .. } = hw;

    use embedded::interfaces::gpio::{Gpio};

    let mut gpio = Gpio::new(gpio_a,
                             gpio_b,
                             gpio_c,
                             gpio_d,
                             gpio_e,
                             gpio_f,
                             gpio_g,
                             gpio_h,
                             gpio_i,
                             gpio_j,
                             gpio_k);

    system_clock::init(rcc, pwr, flash);

    // enable all gpio ports
    rcc.ahb1enr.update(|r| {
        r.set_gpioaen(true);
        r.set_gpioben(true);
        r.set_gpiocen(true);
        r.set_gpioden(true);
        r.set_gpioeen(true);
        r.set_gpiofen(true);
        r.set_gpiogen(true);
        r.set_gpiohen(true);
        r.set_gpioien(true);
        r.set_gpiojen(true);
        r.set_gpioken(true);
    });

    // init sdram (needed for display buffer)
    sdram::init(rcc, fmc, &mut gpio);

    // lcd controller
    let mut lcd = lcd::init(ltdc, rcc, &mut gpio);
    lcd.clear_screen();
    lcd.set_background_color(lcd::Color::rgb(0,0,0));

    let film :Film = Film {
        x_resolution: 480,
        y_resolution: 272,
        supersampling: 2,
        color: Vec3::new(0.0,0.4,0.8),
        iso: 100,
    };

    /*
     * let cam = PerspectiveCamera::new(
     *     Vec3::new(-4.0,0.0,0.5),
     *     Vec3::zero(),
     *     film
     * );
     */

    let cam = OrthographicCamera::new(
        Vec3::new(-154.0,0.0,0.5),
        Vec3::zero(),
        film
    );

    let mut display = LcdDisplay::init(lcd);

    render(&mut display, &cam, &SCENE_SPHERE);

    loop {
    }
}

