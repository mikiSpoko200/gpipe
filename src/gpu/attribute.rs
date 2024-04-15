use nalgebra::{SVector, Scalar};

pub trait Attribute: Default + Scalar {}

macro_rules! impl_attribute {
    ($tt: ty) => {
        impl Attribute for $tt {}
    }
}

impl_attribute!(f32);
impl_attribute!(SVector<f32, 1>);
impl_attribute!(SVector<f32, 2>);
impl_attribute!(SVector<f32, 3>);
impl_attribute!(SVector<f32, 4>);
