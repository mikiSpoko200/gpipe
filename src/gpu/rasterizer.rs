use std::marker::PhantomData;

use super::primitive::Primitive;

pub struct Configuration {
    pub n_samples: usize,
}

pub struct Rasterizer<P>
where
    P: Primitive
{
    _phantom: PhantomData<P>
}

impl<P> Rasterizer<P>
where
    P: Primitive
{
    pub fn interpolate() {
        
    }
}