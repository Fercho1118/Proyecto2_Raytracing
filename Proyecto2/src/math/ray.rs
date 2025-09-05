// Estructura Ray para raytracing

use super::Vec3;

#[derive(Debug, Clone)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Ray {
            origin,
            direction: direction.normalize(),
        }
    }

    /// Obtiene un punto a lo largo del rayo en el parámetro t
    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }
}

impl std::fmt::Display for Ray {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Ray {{ origin: {}, direction: {} }}", self.origin, self.direction)
    }
}