use minifb::{Window, WindowOptions};
use tiny_skia::Pixmap;

/// Owns the OS window and the pixel buffer we draw into.
pub struct Canvas {
    window: Window,
    pixmap: Pixmap,
    /// Reused each frame: pixmap RGBA → minifb `0x00RRGGBB` u32 pixels.
    present_buf: Vec<u32>,
}

impl Canvas {
    /// Open a window of the given size and allocate a matching pixmap.
    pub fn new(width: u32, height: u32, title: &str) -> Self {
        let window = Window::new(
            title,
            width as usize,
            height as usize,
            WindowOptions::default(),
        )
        .expect("failed to open window");

        let pixmap = Pixmap::new(width, height).expect("failed to allocate pixmap");
        let present_buf = vec![0; (width * height) as usize];

        Self {
            window,
            pixmap,
            present_buf,
        }
    }

    /// Borrow the pixmap mutably so draw functions can paint into it.
    pub fn pixmap_mut(&mut self) -> &mut Pixmap {
        &mut self.pixmap
    }

    /// Convert the pixmap's RGBA bytes to the `0x00RRGGBB` u32 format
    /// that minifb expects, then push the buffer to the OS window.
    pub fn present(&mut self) {
        let (w, h) = (self.pixmap.width() as usize, self.pixmap.height() as usize);
        let pixels = self.pixmap.data();

        // tiny-skia stores pixels as premultiplied RGBA; for fully-opaque
        // shapes the channel values equal their straight-alpha equivalents.
        for (dst, rgba) in self
            .present_buf
            .iter_mut()
            .zip(pixels.as_chunks::<4>().0.iter())
        {
            *dst = ((rgba[0] as u32) << 16) | ((rgba[1] as u32) << 8) | rgba[2] as u32;
        }

        self.window
            .update_with_buffer(&self.present_buf, w, h)
            .expect("failed to present buffer");
    }

    /// Returns false once the user closes the window or presses Escape.
    pub fn is_open(&self) -> bool {
        self.window.is_open() && !self.window.is_key_down(minifb::Key::Escape)
    }
}
