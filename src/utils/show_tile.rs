use sdl2::{
    event::Event,
    keyboard::Keycode,
    render::{Canvas, Texture},
    video::Window,
};

use crate::{
    cartridge::rom::ROM,
    rendering::{frame::Frame, SYSTEM_PALLETE},
};

fn show_tile_bank(chr_rom: &Vec<u8>, bank: usize) -> Frame {
    assert!(bank <= 1);

    let mut frame = Frame::new();
    let mut tile_y = 0;
    let mut tile_x = 0;

    let bank = (bank * 0x1000) as usize;

    for tile_n in 0..255 {
        if tile_n != 0 && tile_n % 20 == 0 {
            tile_y += 10;
            tile_x = 0;
        }
        let tile = &chr_rom[(bank + tile_n * 16)..=(bank + tile_n * 16 + 15)];

        for y in 0..=7 {
            let mut upper = tile[y];
            let mut lower = tile[y + 8];

            for x in (0..=7).rev() {
                let value = (1 & upper) << 1 | (1 & lower);
                upper = upper >> 1;
                lower = lower >> 1;
                // this random choices.
                let rgb = match value {
                    0 => SYSTEM_PALLETE[0x01],
                    1 => SYSTEM_PALLETE[0x23],
                    2 => SYSTEM_PALLETE[0x27],
                    3 => SYSTEM_PALLETE[0x30],
                    _ => panic!("can't be"),
                };
                frame.set_pixel(tile_x + x, tile_y + y, rgb);
            }
        }
        tile_x += 10;
    }
    frame
}

pub fn show_tile_viewer(
    mut canvas: Canvas<Window>,
    mut texture: Texture<'_>,
    mut event_pump: sdl2::EventPump,
    nes_file_name: String,
) {
    let bytes: Vec<u8> = std::fs::read(nes_file_name.as_str()).unwrap();
    let rom = ROM::new(&bytes).unwrap();
    let tile_frame = show_tile_bank(&rom.chr_rom, 1);
    texture.update(None, &tile_frame.data, 256 * 3).unwrap();
    canvas.copy(&texture, None, None).unwrap();
    canvas.present();

    loop {
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
    }
}
