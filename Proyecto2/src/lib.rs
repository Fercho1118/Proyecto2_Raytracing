// Declaración de todos los módulos del proyecto

pub mod math;
pub mod material;
pub mod geometry;
pub mod lighting;
pub mod camera;
pub mod scene;
pub mod raytracer;
pub mod texture;

// Re-exportar las estructuras principales 
pub use math::{Vec3, Ray};
pub use material::Material;
pub use geometry::{Plane, Cube, HitRecord};
pub use lighting::Light;
pub use camera::Camera;
pub use scene::Scene;
pub use raytracer::Raytracer;
pub use texture::Texture;

// Constantes globales
pub const EPSILON: f32 = 0.001;
pub const MAX_DEPTH: i32 = 5;