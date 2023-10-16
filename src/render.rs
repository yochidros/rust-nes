use crate::{
    ppu::NesPPU,
    rendering::{
        frame::Frame,
        render_util::{bg_pallette, sprite_pallete},
        SYSTEM_PALLETE,
    },
};

pub fn render(ppu: &NesPPU, frame: &mut Frame) {
    let bank = ppu.control_reg.background_pattern_addr();

    println!("bank {:x}", bank);

    for i in 0..0x03c0 {
        let tile = ppu.vram[i] as u16;
        let tile_column = i % 32;
        let tile_row = i / 32;
        let s = (bank + tile * 16) as usize;
        let e = (bank + tile * 16 + 15) as usize;
        let tile = &ppu.chr_rom[s..=e];
        let palette = bg_pallette(ppu, tile_column, tile_row);
        for y in 0..=7 {
            let mut upper = tile[y];
            let mut lower = tile[y + 8];

            for x in (0..=7).rev() {
                let value = (1 & lower) << 1 | (1 & upper);
                upper = upper >> 1;
                lower = lower >> 1;
                let rgb = match value {
                    0 => SYSTEM_PALLETE[ppu.palette_table[0] as usize],
                    1 => SYSTEM_PALLETE[palette[1] as usize],
                    2 => SYSTEM_PALLETE[palette[2] as usize],
                    3 => SYSTEM_PALLETE[palette[3] as usize],
                    _ => panic!("can't be"),
                };
                frame.set_pixel(tile_column * 8 + x, tile_row * 8 + y, rgb);
            }
        }
    }

    for i in (0..ppu.oam_data.len()).step_by(4).rev() {
        let tile_idx = ppu.oam_data[i + 1] as u16;
        let tile_x = ppu.oam_data[i + 3] as usize;
        let tile_y = ppu.oam_data[i] as usize;

        let flip_vertical = if ppu.oam_data[i + 2] >> 7 & 1 == 1 {
            true
        } else {
            false
        };
        let flip_horizontal = if ppu.oam_data[i + 2] >> 6 & 1 == 1 {
            true
        } else {
            false
        };

        let pallette_idx = ppu.oam_data[i + 2] & 0b11;
        let sprite_palette = sprite_pallete(ppu, pallette_idx);
        let bank: u16 = ppu.control_reg.sprite_pattern_addr();

        let start = (bank + tile_idx * 16) as usize;
        let end = (bank + tile_idx * 16 + 15) as usize;
        let tile = &ppu.chr_rom[start..=end];

        for y in 0..=7 {
            let mut upper = tile[y];
            let mut lower = tile[y + 8];
            'ololo: for x in (0..=7).rev() {
                let value = (1 & lower) << 1 | (1 & upper);
                upper = upper >> 1;
                lower = lower >> 1;
                let rgb = match value {
                    0 => continue 'ololo,
                    1 => SYSTEM_PALLETE[sprite_palette[1] as usize],
                    2 => SYSTEM_PALLETE[sprite_palette[2] as usize],
                    3 => SYSTEM_PALLETE[sprite_palette[3] as usize],
                    _ => panic!("can't be"),
                };
                match (flip_horizontal, flip_vertical) {
                    (false, false) => frame.set_pixel(tile_x + x, tile_y + y, rgb),
                    (true, false) => frame.set_pixel(tile_x + 7 - x, tile_y + y, rgb),
                    (false, true) => frame.set_pixel(tile_x + x, tile_y + 7 - y, rgb),
                    (true, true) => frame.set_pixel(tile_x + 7 - x, tile_y + 7 - y, rgb),
                };
            }
        }
    }
}
