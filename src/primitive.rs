use nalgebra_glm as glm;

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Point {
    position: glm::Vec3
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Line {
    v1: glm::Vec3,
    v2: glm::Vec3,
}

impl Line {
    pub fn new(v1: glm::Vec3, v2: glm::Vec3) -> Self {
        Self {
            v1, v2
        }
    }

    pub fn v1(&self) -> &glm::Vec3 {
        &self.v1
    }

    pub fn v2(&self) -> &glm::Vec3 {
        &self.v2
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Triangle {
    v1: glm::Vec3,
    v2: glm::Vec3,
    v3: glm::Vec3,
}

impl Triangle {
    pub fn v1(&self) -> &glm::Vec3 {
        &self.v1
    }

    pub fn v2(&self) -> &glm::Vec3 {
        &self.v2
    }

    pub fn v3(&self) -> &glm::Vec3 {
        &self.v3
    }
}