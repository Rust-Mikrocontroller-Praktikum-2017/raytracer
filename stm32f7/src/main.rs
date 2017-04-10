#![allow(dead_code)]

#![no_std]
#![no_main]

extern crate stm32f7_discovery as stm32f7;

extern crate r0;
extern crate rtlib;

use stm32f7::{system_clock, sdram, lcd, i2c, touch, board, embedded};

mod display;

use rtlib::vector::{Vec3, Vec2};
use rtlib::render::render;
use rtlib::camera::Film;
use rtlib::cameras::orthographic::OrthographicCamera;
use rtlib::scenes::spheres::SCENE_SPHERE;
use rtlib::camera::Axis;
use rtlib::camera::CameraOperations;
use display::LcdDisplay;

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
                          i2c_3,
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

    i2c::init_pins_and_clocks(rcc, &mut gpio);
    let mut i2c_3 = i2c::init(i2c_3);

    touch::check_family_id(&mut i2c_3).unwrap();

    let film :Film = Film {
        x_resolution: 480,
        y_resolution: 272,
        supersampling: 1,
        color: Vec3::new(0.0,0.4,0.8),
        iso: 100,
    };

    /*
     * let cam = PerspectiveCamera::new(
     *     Vec3::new(-49.0,0.0,0.5),
     *     Vec3::zero(),
     *     film
     * );
     */

    let mut cam = OrthographicCamera::new(
        Vec3::new(-154.0,0.0,80.0),
        Vec3::zero(),
        film
    );

    let mut display = LcdDisplay::init(lcd);

    render(&mut display, &cam, &SCENE_SPHERE);

    let mut swipe = Vec2::zero();
    let mut last_touch_time = system_clock::ticks();
    let mut last_touch = Vec2::zero();
    loop {
        if let Ok(mut touches) = touch::touches(&mut i2c_3) {
            if !touches.is_empty() {
                let touch: Vec2;
                match touches.remove(0) {
                    Some(t) => touch = Vec2::new((t.x) as f32, (t.y) as f32),
                    None    => touch = Vec2::zero(),
                }
                let dir = last_touch.sub(&touch);
                if last_touch.length() != 0.0 && dir.max_norm() < 480.0 {
                    last_touch_time = system_clock::ticks();
                    swipe.inplace_add(&dir);
                }
                last_touch = touch;
            }
        }
        if swipe.max_norm() > 10.0 && system_clock::ticks() - last_touch_time > 500 {
            if swipe.u > swipe.v {
                cam.rotate(Axis::Z, swipe.u);
                swipe = Vec2::zero();
            } else {
                cam.rotate(Axis::Y, swipe.v);
                swipe = Vec2::zero();
            }
            render(&mut display, &cam, &SCENE_SPHERE);
            last_touch_time = system_clock::ticks();
        }
    }
}

