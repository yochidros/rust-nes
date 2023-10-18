mod bus;
mod cartridge;
mod cpu_internals;
mod ppu;
mod render;
mod rendering;
mod utils;

use crate::cpu_internals::cpu::CPU;
use cartridge::mem::Mem;
use cartridge::rom::ROM;
use ppu::NesPPU;

use render::render;
use rendering::frame::Frame;
use sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels::{Color, PixelFormatEnum},
    rect::Point,
    EventPump,
};

fn handle_user_input(cpu: &mut CPU, event_pump: &mut EventPump) {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => std::process::exit(0),
            Event::KeyDown {
                keycode: Some(Keycode::W),
                ..
            } => cpu.mem_write(0xff, 0x77),
            Event::KeyDown {
                keycode: Some(Keycode::S),
                ..
            } => cpu.mem_write(0xff, 0x73),
            Event::KeyDown {
                keycode: Some(Keycode::A),
                ..
            } => cpu.mem_write(0xff, 0x61),
            Event::KeyDown {
                keycode: Some(Keycode::D),
                ..
            } => cpu.mem_write(0xff, 0x64),
            _ => {}
        }
    }
}
fn read_screen_state(cpu: &mut CPU, frame: &mut [u8; 32 * 3 * 32]) -> bool {
    let mut frame_idx = 0;
    let mut update = false;
    for i in 0x0200..0x600 {
        let color_idx = cpu.mem_read(i as u16);
        let (b1, b2, b3) = color(color_idx).rgb();
        if frame[frame_idx] != b1 || frame[frame_idx + 1] != b2 || frame[frame_idx + 2] != b3 {
            frame[frame_idx] = b1;
            frame[frame_idx + 1] = b2;
            frame[frame_idx + 2] = b3;
            update = true;
        }
        frame_idx += 3;
    }
    update
}
fn color(byte: u8) -> Color {
    match byte {
        0 => Color::BLACK,
        1 => Color::WHITE,
        2 | 9 => Color::GREY,
        3 | 10 => Color::RED,
        4 | 11 => Color::GREEN,
        5 | 12 => Color::BLUE,
        6 | 13 => Color::MAGENTA,
        7 | 14 => Color::YELLOW,
        _ => Color::CYAN,
    }
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("nes game", (256.0 * 3.0) as u32, (240.0 * 3.0) as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    canvas.set_scale(3.0, 3.0).unwrap();
    let creator = canvas.texture_creator();
    let mut texture = creator
        .create_texture_target(PixelFormatEnum::RGB24, 256, 240)
        .unwrap();

    let args = std::env::args().nth(1).unwrap();
    // show_tile_viewer(canvas, texture, event_pump, args);

    let bytes: Vec<u8> = std::fs::read(args.as_str()).unwrap();
    let rom = ROM::new(&bytes).unwrap();
    let mut frame = Frame::new();

    let bus = bus::Bus::new(rom, move |ppu: &NesPPU| {
        render(ppu, &mut frame);
        texture.update(None, &frame.data, 256 * 3).unwrap();
        canvas.copy(&texture, None, None).unwrap();

        // draw grid lines
        let tmp = canvas.draw_color();
        for i in (8..256).step_by(8) {
            canvas.set_draw_color(color(120));
            canvas.set_blend_mode(sdl2::render::BlendMode::Add);
            canvas
                .draw_line(Point::new(i, 0), Point::new(i, 240))
                .unwrap();
            canvas
                .draw_line(Point::new(0, i - 1), Point::new(256, i - 1))
                .unwrap();
            canvas.set_draw_color(tmp);
        }

        canvas.present();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => std::process::exit(0),
                _ => {}
            }
        }
    });

    let mut cpu = CPU::new(bus);
    cpu.reset();
    cpu.run();
}
