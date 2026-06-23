#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct Vertex {
    pub pos: [f32; 2],
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Ball {
    pub pos: [f32; 2],
    pub radius: f32,
    _padding1: f32,
    pub color: [f32; 3],
    _padding2: f32,
}

impl Ball {
    pub fn new(x: f32, y: f32, radius: f32, color: [f32; 3]) -> Self {
        Self {
            pos: [x, y],
            radius,
            color,
            _padding1: 0.,
            _padding2: 0.,
        }
    }
}
