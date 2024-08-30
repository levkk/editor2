use super::Vertex;

pub trait Figure {
    fn data(&self) -> Vec<Vertex>;
    fn indices(&self) -> Vec<u16>;
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Rectangle {
    p0: Vertex,
    p1: Vertex,
    p2: Vertex,
    p3: Vertex,
}

impl Figure for Rectangle {
    fn data(&self) -> Vec<Vertex> {
        vec![self.p0, self.p1, self.p2, self.p3]
    }

    fn indices(&self) -> Vec<u16> {
        vec![0, 1, 3, 0, 3, 2]
    }
}

impl Rectangle {
    pub fn new(x: f32, y: f32, height: f32) -> Rectangle {
        let half = height / 2.0;
        let x1 = x - half;
        let x2 = x + half;
        let y1 = y - half;
        let y2 = y + half;

        Rectangle {
            p0: Vertex::new(x1, y1),
            p1: Vertex::new(x2, y1),
            p2: Vertex::new(x1, y2),
            p3: Vertex::new(x2, y2),
        }
    }

    pub fn green(mut self) -> Self {
        self.p0 = self.p0.color(0., 1., 0.);
        self.p1 = self.p1.color(0., 1., 0.);
        self.p2 = self.p2.color(0., 1., 0.);
        self.p3 = self.p3.color(0., 1., 0.);
        self
    }
}
