// Módulo matemático

pub mod vec3;
pub mod ray;

pub use vec3::Vec3;
pub use ray::Ray;

// Utilidades matemáticas
pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * std::f32::consts::PI / 180.0
}