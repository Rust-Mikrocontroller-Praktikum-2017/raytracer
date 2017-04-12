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
// use rtlib::cameras::orthographic::OrthographicCamera;
use rtlib::cameras::perspective::PerspectiveCamera;
use rtlib::scenes::{space,spheres,pyramid};
use rtlib::camera::Axis;
use rtlib::camera::CameraOperations;
use rtlib::math::{abs, HALFPI};
use display::LcdDisplay;
use rtlib::display::Display;
use rtlib::textures::file::FileTexture;
use rtlib::textures::color::NoTexture;

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

#[inline(never)]
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

    use embedded::interfaces::gpio::{self, Gpio};

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

    let button_pin = (gpio::Port::PortI, gpio::Pin::Pin11);
    let button = gpio.to_input(button_pin, gpio::Resistor::NoPull)
        .expect("button pin already in use");

    let film_0 :Film = Film {
        x_resolution: 480,
        y_resolution: 272,
        supersampling: 1,
        texture: &NoTexture { color: Vec3 { x: 0.0, y: 0.1, z: 0.2 } },
        iso: 100,
    };

    let cam_0 = PerspectiveCamera::new(
        Vec3::new(-200.0,0.0,10.0),
        Vec3::new(0.0,0.0,0.0),
        film_0
    );

    let film_1 :Film = Film {
        x_resolution: 480,
        y_resolution: 272,
        supersampling: 1,
        texture: &FileTexture{
            width: 403,
            height: 161,
            rgbdata: include_bytes!("../../textures/sky.rgb")
        },
        iso: 100,
    };

    let mut cam_1 = PerspectiveCamera::new(
        Vec3::new(-200.0,100.0,150.0),
        Vec3::new(0.0,0.0,20.0),
        film_1
    );

    cam_1.set_field_of_view(45);

    /*
     * let mut cam = OrthographicCamera::new(
     *     Vec3::new(-154.0,0.0,80.0),
     *     Vec3::zero(),
     *     film
     * );
     */

    let mut display = LcdDisplay::init(lcd);
    let mut cams = [cam_1,cam_0.clone(),cam_0];
    let scenes = [pyramid::SCENE_PYRAMID, spheres::SCENE_SPHERE, space::SCENE_SPACE];
    let mut current_scene = 0;

    render(&mut display, &cams[current_scene], &scenes[current_scene]);

    let mut swipe = Vec2::zero();
    let mut last_touch_time = system_clock::ticks();
    let mut last_touch = Vec2::zero();
    let mut button_pressed_old = false;
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
        if system_clock::ticks() - last_touch_time > 500 {
            if swipe.max_norm() > 10.0 {
                if abs(swipe.u) > abs(swipe.v) {
                    let x_res = cams[current_scene].film.x_resolution as f32;
                    let rad = swipe.u*1.25/x_res*HALFPI;
                    cams[current_scene].rotate(Axis::Z, rad);
                } else {
                    let y_res = cams[current_scene].film.y_resolution as f32;
                    let rad = -swipe.v*1.25/y_res*HALFPI;
                    cams[current_scene].rotate(Axis::Y, rad);
                }
                display.reset();
                render(&mut display, &cams[current_scene], &scenes[current_scene]);
            }
            swipe = Vec2::zero();
            last_touch = Vec2::zero();
            last_touch_time = system_clock::ticks();
        }

        let button_pressed = button.get();
        if button_pressed && !button_pressed_old {
            current_scene += 1;
            current_scene %= scenes.len();
            display.reset();
            render(&mut display, &cams[current_scene], &scenes[current_scene]);
        }

        button_pressed_old = button_pressed;
    }
}

