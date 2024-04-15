use nalgebra::SVector;
use std::ops::{Index, IndexMut, Range};

use super::attribute::Attribute;

pub struct Buffer<T, const N_COMPONENTS: usize> {
    id: usize,
    buffer: Box<[SVector<T, N_COMPONENTS>]>,
}

impl<T, const N_COMPONENTS: usize> Buffer<T, N_COMPONENTS> {
    pub(super) fn new(id: usize, length: usize) -> Self
where
    T: Attribute,
    [T; N_COMPONENTS]: Default
{
        Self {
            id,
            buffer: vec![SVector::default(); length].into_boxed_slice(),
        }
    }
}

impl<T, const N_COMPONENTS: usize> Index<usize> for Buffer<T, N_COMPONENTS> {
    type Output = SVector<T, N_COMPONENTS>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.buffer[index]
    }
}

impl<T, const N_COMPONENTS: usize> IndexMut<usize> for Buffer<T, N_COMPONENTS> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.buffer[index]
    }
}

impl<T, const N_COMPONENTS: usize> Index<Range<usize>> for Buffer<T, N_COMPONENTS> {
    type Output = [SVector<T, N_COMPONENTS>];

    fn index(&self, index: Range<usize>) -> &Self::Output {
        &self.buffer[index]
    }
}

impl<T, const N_COMPONENTS: usize> IndexMut<Range<usize>> for Buffer<T, N_COMPONENTS> {
    fn index_mut(&mut self, index: Range<usize>) -> &mut Self::Output {
        &mut self.buffer[index]
    }
}

pub mod untyped {
    use nalgebra::SVector;

    pub enum Buffer {
        Vec1f32(Box<[SVector<f32, 1>]>),
        Vec2f32(Box<[SVector<f32, 2>]>),
        Vec3f32(Box<[SVector<f32, 3>]>),
        Vec4f32(Box<[SVector<f32, 4>]>),
    }

    // impl<const N_COMPONENTS: usize> From<Box<SVector<f32, N_COMPONENTS>>> for Buffer {
    //     fn from(value: Box<SVector<f32, N_COMPONENTS>>) -> Self {
            
    //     }
    // }
}