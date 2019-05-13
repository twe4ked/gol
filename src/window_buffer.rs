pub struct WindowBuffer {
    pub buffer: Vec<u32>,
    width: usize,
    height: usize,
}

impl WindowBuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            buffer: vec![0; width * height],
            width,
            height,
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: u32) {
        let l = y * self.width + x;
        assert!(l <= self.width * self.height - 2);
        self.buffer[l] = color;
    }

    pub fn clear(&mut self) {
        self.buffer = vec![0; self.width * self.height];
    }
}
