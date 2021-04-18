use core::mem::MaybeUninit;

use crate::graphics::{Graphics, PixelColor};

static mut RAW_CONSOLE: MaybeUninit<Console> = MaybeUninit::<Console>::uninit();

pub const ROWS: usize = 25;
pub const COLUMNS: usize = 80;
pub const LINE_HEIGHT: usize = 18;

#[derive(Debug, Copy, Clone)]
pub struct Console {
    buffer: [[char; COLUMNS + 1]; ROWS],
    fg_color: PixelColor,
    bg_color: PixelColor,
    cursor_row: usize,
    cursor_column: usize,
    buffer_row_offset: usize,
}

impl Console {
    fn new(fg_color: &PixelColor, bg_color: &PixelColor) -> Self {
        Console {
            buffer: [[0.into(); COLUMNS + 1]; ROWS],
            fg_color: *fg_color,
            bg_color: *bg_color,
            cursor_row: 0,
            cursor_column: 0,
            buffer_row_offset: 0,
        }
    }

    pub fn initialize(fg_color: &PixelColor, bg_color: &PixelColor) {
        unsafe { core::ptr::write(RAW_CONSOLE.as_mut_ptr(), Console::new(fg_color, bg_color)) };
    }

    pub fn instance() -> &'static mut Console {
        unsafe { &mut *RAW_CONSOLE.as_mut_ptr() }
    }

    pub fn actual_row(&self, row: usize) -> usize {
        (row + self.buffer_row_offset) % ROWS
    }

    pub fn actual_cursor_row(&self) -> usize {
        self.actual_row(self.cursor_row)
    }

    pub fn newline(&mut self, graphics: &mut Graphics) {
        self.cursor_column = 0;
        if self.cursor_row < ROWS - 1 {
            self.cursor_row += 1;
        } else {
            // clear
            for y in 0..(ROWS * LINE_HEIGHT) {
                for x in 0..(COLUMNS * 8) {
                    graphics.write_pixel(x, y, &self.bg_color);
                }
            }
            self.buffer_row_offset = (self.buffer_row_offset + 1) % ROWS;
            for row in 0..(ROWS - 1) {
                for column in 0..(COLUMNS - 1) {
                    graphics.write_ascii(
                        8 * column,
                        LINE_HEIGHT * row,
                        self.buffer[self.actual_row(row)][column],
                        &self.fg_color,
                    );
                }
            }
            self.buffer[self.actual_cursor_row()] = [0.into(); COLUMNS + 1];
        }
    }
    pub fn put_string(&mut self, s: &str) {
        let graphics = Graphics::instance();
        for c in s.chars() {
            if c == '\n' {
                self.newline(graphics);
            }
            if self.cursor_column < COLUMNS && c as u32 >= 0x20 {
                graphics.write_ascii(
                    8 * self.cursor_column,
                    LINE_HEIGHT * self.cursor_row,
                    c,
                    &self.fg_color,
                );
                self.buffer[self.actual_cursor_row()][self.cursor_column] = c;
                self.cursor_column += 1;
            }
        }
    }
}
