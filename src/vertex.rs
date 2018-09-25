
#[derive(Copy, Clone)]
pub struct Vertex {
    position: (f32, f32, f32),
    normal:   (f32, f32, f32)
}
impl Vertex {
    pub fn new(x: f32, y: f32, z: f32, normal: (f32, f32, f32)) -> Self {
        Vertex {
            position: (x, y, z),
            normal   
        }
    }
}

implement_vertex!(Vertex, position, normal);