use super::figures::Figure;
use super::Vertex;

#[derive(Debug)]
pub struct Scene {
    vertices: Vec<Vertex>,
    indices: Vec<u16>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            vertices: vec![],
            indices: vec![],
        }
    }

    pub fn add(&mut self, figure: impl Figure) {
        let last_index = self.vertices.len();
        self.vertices.extend(figure.data());
        let indices = figure
            .indices()
            .into_iter()
            .map(|i| i + last_index as u16)
            .collect::<Vec<_>>();
        self.indices.extend(indices);
    }

    pub fn data(&self) -> &[Vertex] {
        &self.vertices
    }

    pub fn indices(&self) -> &[u16] {
        &self.indices
    }
}
