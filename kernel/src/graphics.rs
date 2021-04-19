use core::mem::MaybeUninit;

use uefi::proto::console::gop::{FrameBuffer, ModeInfo, PixelFormat};

use crate::ascii_font::FONTS;

#[derive(Copy, Clone, Debug)]
pub struct PixelColor(pub u8, pub u8, pub u8); // RGB

// static singleton pointer
static mut RAW_GRAPHICS: MaybeUninit<Graphics> = MaybeUninit::<Graphics>::uninit();
static mut GRAPHICS_INITIALIZED: bool = false;

pub struct Graphics<'a> {
    fb: &'a mut FrameBuffer<'a>,
    mi: ModeInfo,
    pixel_writer: unsafe fn(&mut FrameBuffer, usize, &PixelColor),
}

impl<'a> Graphics<'a> {
    pub fn new(fb: &'a mut FrameBuffer<'a>, mi: ModeInfo) -> Self {
        unsafe fn write_pixel_rgb(fb: &mut FrameBuffer, index: usize, rgb: &PixelColor) {
            fb.write_value(index, [rgb.0, rgb.1, rgb.2]);
        }
        unsafe fn write_pixel_bgr(fb: &mut FrameBuffer, index: usize, rgb: &PixelColor) {
            fb.write_value(index, [rgb.2, rgb.1, rgb.0]);
        }
        let pixel_writer = match mi.pixel_format() {
            PixelFormat::Rgb => write_pixel_rgb,
            PixelFormat::Bgr => write_pixel_bgr,
            _ => {
                panic!("This pixel format is not supported by the drawing demo");
            }
        };

        Graphics {
            fb,
            mi,
            pixel_writer,
        }
    }

    pub fn instance() -> &'static mut Self {
        if unsafe { !GRAPHICS_INITIALIZED } {
            panic!("graphics not initialized");
        }
        unsafe { &mut *RAW_GRAPHICS.as_mut_ptr() }
    }

    ///
    /// # Safety
    /// This is unsafe : handle raw pointers.
    pub unsafe fn initialize_instance(fb: *mut FrameBuffer<'static>, mi: *mut ModeInfo) {
        core::ptr::write(RAW_GRAPHICS.as_mut_ptr(), Graphics::new(&mut *fb, *mi));
        GRAPHICS_INITIALIZED = true;
    }

    /// Write to the pixel of the buffer
    ///
    pub fn write_pixel(&mut self, x: usize, y: usize, color: &PixelColor) {
        // TODO: don't panic.
        let (width, height) = self.mi.resolution();
        if x > width {
            panic!("bad x coord");
        }
        if y > height {
            panic!("bad x coord");
        }
        let pixel_index = y * self.mi.stride() + x;
        let base = 4 * pixel_index;
        unsafe {
            (self.pixel_writer)(&mut self.fb, base, color);
        }
    }

    pub fn write_ascii(&mut self, x: usize, y: usize, c: char, color: &PixelColor) {
        if (c as u32) > 0x7f {
            return;
        }
        let font: [u8; 16] = FONTS[c as usize];
        for (dy, line) in font.iter().enumerate() {
            for dx in 0..8 {
                if (line << dx) & 0x80 != 0 {
                    self.write_pixel(x + dx, y + dy, &color);
                }
            }
        }
    }

    pub fn write_string(
        &mut self,
        mut x: usize,
        mut y: usize,
        str: &str,
        color: &PixelColor,
    ) -> (usize, usize) {
        let first_x = x;
        let (width, height) = self.resolution();
        for c in str.chars() {
            self.write_ascii(x, y, c, color);
            x += 8;
            if x > width {
                x = first_x;
                y += 20;
            }
            if y > height {
                // can not write
                return (x, y);
            }
        }
        (x, y)
    }

    pub fn resolution(&self) -> (usize, usize) {
        self.mi.resolution()
    }

    pub fn clear(&mut self, color: &PixelColor) {
        let (width, height) = self.resolution();
        for y in 0..height {
            for x in 0..width {
                self.write_pixel(x, y, color);
            }
        }
    }

    pub fn text_writer(
        &'a mut self,
        first_x: usize,
        first_y: usize,
        color: &PixelColor,
    ) -> TextWriter {
        TextWriter::new(self, first_x, first_y, color)
    }
}

pub struct TextWriter<'a> {
    graphics: &'a mut Graphics<'a>,
    first_x: usize,
    first_y: usize,
    x: usize,
    y: usize,
    color: PixelColor,
}

impl<'a> TextWriter<'a> {
    pub fn new(
        graphics: &'a mut Graphics<'a>,
        first_x: usize,
        first_y: usize,
        color: &PixelColor,
    ) -> Self {
        TextWriter {
            graphics,
            first_x,
            first_y,
            x: first_x,
            y: first_y,
            color: *color,
        }
    }

    pub fn reset_coord(&mut self) {
        self.x = self.first_x;
        self.y = self.first_y;
    }

    pub fn change_color(&mut self, color: &PixelColor) {
        self.color = *color;
    }
}

impl<'a> core::fmt::Write for TextWriter<'a> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let (x, y) = self.graphics.write_string(self.x, self.y, s, &self.color);
        self.x = x;
        self.y = y;
        Ok(())
    }
}
