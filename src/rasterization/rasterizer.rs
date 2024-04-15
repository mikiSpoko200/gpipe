use std::marker::PhantomData;

use super::Fragment;

pub trait Rasterizer<P> {
    fn fragments(&self) -> impl Iterator<Item=Fragment>;
}

pub trait LineRasterizer {
    fn fragments(&self) -> impl Iterator<Item=Fragment>;
}

pub struct BresenhamLineRasterizer;

pub struct FastBresenhamLineRasterizer;

pub struct ElipsisRasterizer;

pub struct PointRasterizer;

pub struct TriangleRasterizer<LR: LineRasterizer>(PhantomData<LR>);
