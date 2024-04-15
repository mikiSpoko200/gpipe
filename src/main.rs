pub mod primitive;
pub mod buffer;
pub mod rasterization;
pub mod fragment_post_processing;
pub mod vertex_post_processing;

use nalgebra_glm as glm;

type Framebuffer = buffer::Buffer<glm::Vec3>;
type ZBuffer = buffer::Buffer<f32>;

impl ZBuffer {
    pub fn depth_test(&self, screen_space_position: &glm::Vec3) -> bool {
        todo!()
    }
}

fn main() {
    
}
