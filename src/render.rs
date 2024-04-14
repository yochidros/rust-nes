use std::usize;

use crate::{
    ppu::{register::status_register::StatusRegister, NesPPU, PPUMirroring},
    rendering::{
        frame::Frame,
        rect::Rect,
        render_util::{bg_pallette, sprite_pallete},
        SYSTEM_PALLETE,
    },
};

pub fn render_name_table(
    ppu: &NesPPU,
    frame: &mut Frame,
    name_table: &[u8],
    view_port: Rect,
    shift_x: isize,
    shift_y: isize,
    draw_rect: &Rect,
    scanlines: usize,
) {
    let bank = ppu.control_reg.background_pattern_addr();

    let attrs_table = &name_table[0x3c0..=0x3ff];

    let start_scanline = draw_rect.y1;
    let end_scanline = draw_rect.y2;
    // 960 tiles for background
    for i in 0x0..0x3c0 {
        //0x3c0 {
        let tile_column = i % 32;
        let tile_row = i / 32;
        // Check if the current tile row is within the specified scanline range
        if tile_row * 8 < start_scanline || tile_row * 8 > end_scanline {
            continue; // Skip rendering this tile if it's outside the scanline range
        }

        let tile_idx = name_table[i] as u16;

        let start = (bank + tile_idx * 16) as usize;
        let end = (bank + tile_idx * 16 + 15) as usize;
        let tile = &ppu.chr_rom[start..=end];
        let palette = bg_pallette(ppu, attrs_table, tile_column, tile_row);

        for y in 0..8 {
            let mut upper = tile[y];
            let mut lower = tile[y + 8];

            for x in (0..8).rev() {
                let value = (1 & lower) << 1 | (1 & upper);
                upper >>= 1;
                lower >>= 1;
                let rgb = match value {
                    0 => SYSTEM_PALLETE[ppu.palette_table[0] as usize],
                    1 => SYSTEM_PALLETE[palette[1] as usize],
                    2 => SYSTEM_PALLETE[palette[2] as usize],
                    3 => SYSTEM_PALLETE[palette[3] as usize],
                    _ => panic!("can't be"),
                };
                let pixel_x = tile_column * 8 + x;
                let pixel_y = tile_row * 8 + y;

                if pixel_x >= view_port.x1
                    && pixel_x < view_port.x2
                    && pixel_y >= view_port.y1
                    && pixel_y < view_port.y2
                {
                    let x = (shift_x + pixel_x as isize) as usize;
                    let y = (shift_y + pixel_y as isize) as usize;
                    if x >= draw_rect.x1
                        && x < draw_rect.x2
                        && y >= draw_rect.y1
                        && y < draw_rect.y2
                    {
                        frame.set_pixel(x, y - 8, rgb)
                    }
                }
            }
        }
    }
}
pub fn render(ppu: &NesPPU, frame: &mut Frame, scanlines: u16) {
    let scroll_x = (ppu.scroll_register.scroll_x) as usize;
    let scroll_y = (ppu.scroll_register.scroll_y) as usize;
    let fine_scroll_x = ppu.scroll_register.fine_scroll_x as usize;
    let fine_scroll_y = ppu.scroll_register.fine_scroll_y as usize;
    let adjusted_scroll_x = scroll_x + fine_scroll_x;
    let adjusted_scroll_y = scroll_y + fine_scroll_y;

    let extra_tile_x = if fine_scroll_x > 0 { 1 } else { 0 };
    let extra_tile_y = if fine_scroll_y > 0 { 1 } else { 0 };

    let (main_nametable, second_nametable) =
        match (&ppu.mirroring, ppu.control_reg.nametable_addr()) {
            (PPUMirroring::Vertical, 0x2000)
            | (PPUMirroring::Vertical, 0x2800)
            | (PPUMirroring::Horizontal, 0x2000)
            | (PPUMirroring::Horizontal, 0x2400) => (&ppu.vram[0..0x400], &ppu.vram[0x400..0x800]),
            (PPUMirroring::Vertical, 0x2400)
            | (PPUMirroring::Vertical, 0x2c00)
            | (PPUMirroring::Horizontal, 0x2800)
            | (PPUMirroring::Horizontal, 0x2c00) => (&ppu.vram[0x400..0x800], &ppu.vram[0..0x400]),
            _ => {
                panic!("not implemented yet")
            }
        };
    let draw_rect = Rect::new(0, scanlines as usize, 256, scanlines as usize + 8);

    render_name_table(
        ppu,
        frame,
        main_nametable,
        Rect::new(
            scroll_x,
            scroll_y,
            256 + 8 * extra_tile_x,
            240 + 8 * extra_tile_y,
        ),
        -(scroll_x as isize),
        -(scroll_y as isize),
        &draw_rect,
        scanlines as usize,
    );
    if scroll_x > 0 {
        render_name_table(
            ppu,
            frame,
            second_nametable,
            Rect::new(0, 0, scroll_x, 240),
            (256 - scroll_x) as isize,
            0,
            &draw_rect,
            scanlines as usize,
        );
    } else if scroll_y > 0 {
        render_name_table(
            ppu,
            frame,
            second_nametable,
            Rect::new(0, 0, 256, scroll_y),
            0,
            (240 - scroll_y) as isize,
            &draw_rect,
            scanlines as usize,
        );
    }

    // display sprite
    let bank: u16 = ppu.control_reg.sprite_pattern_addr();
    for i in (0..ppu.oam_data.len()).step_by(4).rev() {
        let mut tile_y = ppu.oam_data[i] as usize;
        let tile_idx = ppu.oam_data[i + 1] as u16;
        let attrs = ppu.oam_data[i + 2];
        let mut tile_x = ppu.oam_data[i + 3] as usize;

        let flip_vertical = attrs >> 7 == 1;
        let flip_horizontal = attrs >> 6 & 0b01 == 1;

        let pallette_idx = attrs & 0b11;
        let sprite_palette = sprite_pallete(ppu, pallette_idx);

        let start = (bank + tile_idx * 16) as usize;
        let end = (bank + tile_idx * 16 + 15) as usize;
        let tile = &ppu.chr_rom[start..=end];
        if tile_y == 0 && tile_x == 0 {
            continue;
        }
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
                    (false, false) => frame.set_pixel(tile_x + x, tile_y + y - 8, rgb),
                    (true, false) => frame.set_pixel(tile_x + 7 - x, tile_y + y - 8, rgb),
                    (false, true) => frame.set_pixel(tile_x + x, tile_y + 7 - y - 8, rgb),
                    (true, true) => frame.set_pixel(tile_x + 7 - x, tile_y + 7 - y - 8, rgb),
                };
            }
        }
    }
}
