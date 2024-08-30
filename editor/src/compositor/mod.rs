use crate::gl::{Scene, Vertex};
use wgpu::SurfaceConfiguration;

#[derive(Copy, Clone)]
pub struct WindowDimensions {
    width: u32,
    height: u32,
}

impl From<&SurfaceConfiguration> for WindowDimensions {
    fn from(config: &SurfaceConfiguration) -> Self {
        Self {
            width: config.width.max(1),
            height: config.height.max(1),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Position {
    Pixel { x: u32, y: u32 },

    End { y: u32 },
    Start { y: u32 },
    Top { x: u32 },
    Bottom { x: u32 },
}

impl Position {
    pub fn to_gl(&self, config: WindowDimensions) -> Vertex {
        use Position::*;

        let w = config.width as f32;
        let h = config.height as f32;

        let midW = w / 2.;
        let midH = h / 2.;

        let map_x = |x: u32, w: f32| {
            let x = x as f32;

            if x < midW {
                -x / midW
            } else {
                (x - midW) / midW
            }
        };

        let map_y = |y: u32, w: f32| {
            let y = y as f32;

            if y < midH {
                -y / midH
            } else {
                (y - midH) / midH
            }
        };

        match self {
            Pixel { x, y } => Vertex::new(map_x(*x, w), map_y(*y, h)),

            End { y } => {
                let x = 1.0;
                let y = map_y(*y, h);

                Vertex::new(x, y)
            }

            _ => todo!(),
        }
    }
}

pub struct Compositor {}

impl Compositor {
    pub fn scene(&self) -> Scene {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_to_gl() {
        let config = WindowDimensions {
            width: 500,
            height: 300,
        };

        let pos = Position::Pixel { x: 250, y: 150 };

        let vertex = pos.to_gl(config);
        assert_eq!(vertex.x(), 0.0);
        assert_eq!(vertex.y(), 0.0);

        let pos = Position::End { y: 300 };

        let vertex = pos.to_gl(config);
        assert_eq!(vertex.x(), 1.0);
        assert_eq!(vertex.y(), 1.0);
    }
}
