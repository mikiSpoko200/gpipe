pub mod rasterizer;
pub mod input_assembler;
pub mod buffer;
pub mod attribute;
pub mod shader;
pub mod program;
pub mod primitive;

use std::marker::PhantomData;

use buffer::Buffer;
use nalgebra::SVector;
use shader::Shader;
use primitive::Primitive;


const N_COLOR_COMPONENTS: usize = 4;
const N_DEPTH_COMPONENTS: usize = 1;

type Framebuffer = Buffer<f32, N_COLOR_COMPONENTS>;
type ZBuffer = Buffer<f32, N_DEPTH_COMPONENTS>;
type Position = [f32; 4];

// TODO: emulate screen tearing with threading and multithreading

pub struct Confiugration {
    backface_culling: bool
}

impl Default for Confiugration {
    fn default() -> Self {
        Self { backface_culling: false }
    }
}

pub struct Pipeline<P, VS, FS, VI, FI>
where
    P: Primitive,
    VS: Shader<VI, FI>,
    FS: Shader<FI, [f32; 4]>,
{
    // vertex processing
    vertex_attributes: Vec<VI>,
    index_buffer: Vec<u32>,
    vertex_output: Vec<FI>,
    vertex_shader: VS,
    fragment_shader: FS,
    _phantoms: PhantomData<P>,
}

pub struct GPU<P, VS, FS, VI, FI>
where
    P: Primitive,
    VS: Shader<VI, FI>,
    FS: Shader<FI, [f32; 4]>,
{
    pipeline: Pipeline<P, VS, FS, VI, FI>,
    buffers: Vec<Box<dyn std::any::Any>>,
    z_buffer: ZBuffer,
    // fragment processing
    framebuffers: [Framebuffer; 2],
    configuration: Confiugration
}

pub enum PrimitiveType {
    Triangle,
    Line,
    Elipsis,
    Point
}

pub trait Allocator<T, const N_COMPONENTS: usize> {
    type Vector;

    fn allocate(&mut self, length: usize) -> &mut Buffer<T, N_COMPONENTS>;
}

impl<P, VS, FS, VI, FI> GPU<P, VS, FS, VI, FI>
where
    P: Primitive,
    VS: Shader<VI, FI>,
    FS: Shader<FI, [f32; 4]>,
{
    fn new(length: usize, pipeline: Pipeline<P, VS, FS, VI, FI>) -> Self {
        Self {
            pipeline,
            buffers: Vec::new(),
            z_buffer: ZBuffer::new(0, length),
            framebuffers: [Framebuffer::new(0, length), Framebuffer::new(1, length)],
            configuration: Confiugration::default(),
        }
    }

    // Problem tutaj bufory różnych typów -- untyped buffer obiekt? Ale potem
    // shader nie będzie mógł mieć ładnego typu -- jakieś makro?
    // krotka list w listę krotek?
    fn vertex_stage<S>(&mut self, attributes: &[&Buffer])
    where
        S: Shader<VI, FI>,
    {
        self.vertex_attributes.iter().for_each(Shader::process)
    }

    fn is_in_screen_space(&self, x: usize, y: usize) -> bool {
        (0..self.framebuffers[0].width).contains(&x) && (0..self.framebuffers[0].height).contains(&y)
    }

    fn clip_to_ndc(&mut self) {

    }

    fn interpolate() {

    }

    fn rasterize() {

    }

    fn depth_test() {
        todo!()
    }

    fn backface_culling() {
        todo!()
    }

    fn draw(primitive_type: PrimitiveType) {
        todo!();
    }
}



/// Allow 
impl<P, VS, FS, VI, FI, const N_COMPONENTS: usize> Allocator<f32, N_COMPONENTS> for GPU<P, VS, FS, VI, FI>
where
    P: Primitive,
    VS: Shader<VI, FI>,
    FS: Shader<FI, [f32; 4]>,
    SVector<f32, N_COMPONENTS>: attribute::Attribute
{
    type Vector = SVector<f32, N_COMPONENTS>;

    fn allocate(&mut self, length: usize) -> &mut Buffer<f32, N_COMPONENTS> {
        let buffer = Box::new(Buffer::new(self.buffers.len(), length));
        let client = &mut *buffer;
        self.buffers.push(
            buffer as Box<_>
        );
        client
    }
}
