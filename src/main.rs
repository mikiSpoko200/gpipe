pub mod gpu;

pub struct Vertex {
    pub position: [f32; 4],
    pub color: [f32; 4],
    pub normal: [f32; 3],
}

pub struct Fragment {
    pub position: [f32; 4],
    pub color: [f32; 4],
    pub normal: [f32; 3],
}

fn main() {
    println!("Hello, world!");
}
