pub const SCREEN_WIDTH: usize = 192;
pub const SCREEN_HEIGHT: usize = 144;
const VIDEO_BUFFER_LEN: usize = SCREEN_WIDTH * SCREEN_HEIGHT;

pub type Pixel = u8;
pub type VideoBuffer = [Pixel; VIDEO_BUFFER_LEN];

#[derive(Debug)]
pub struct VideoDevice {
    pub background: [Pixel; VIDEO_BUFFER_LEN],
    pub foreground: [Pixel; VIDEO_BUFFER_LEN],
}

impl VideoDevice {
    pub fn new() -> Self {
        let mut buffer = [0x00 as Pixel; VIDEO_BUFFER_LEN];
        for i in 0..VIDEO_BUFFER_LEN {
            buffer[i] = (i % 0x10) as Pixel;
        }
        Self {
            background: [0x00; VIDEO_BUFFER_LEN],
            foreground: buffer,
        }
    }
}
