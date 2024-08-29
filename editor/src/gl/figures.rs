
use super::Vertex;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Square {
    p0: Vertex,
    p1: Vertex,
    p2: Vertex,
    p3: Vertex,
}

impl Square {
    pub fn new(x: f32, y: f32, height: f32) -> Square {
        let half = height / 2.0;
        let x1 = x - half;
        let x2 = x + half;
        let y1 = y - half;
        let y2 = y + half;

        Square {
            p0: Vertex::new(x1, y1),
            p1: Vertex::new(x2, y1),
            p2: Vertex::new(x1, y2),
            p3: Vertex::new(x2, y2),
        }
    }

    pub fn data(&self) -> [Vertex; 4] {
        [self.p0, self.p1, self.p2, self.p3]
    }

    pub fn indices(&self) -> [u16; 6] {
        [0, 1, 3, 0, 3, 2]
    }
}