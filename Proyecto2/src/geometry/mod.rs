// Módulo de geometría
pub mod cube;

pub use cube::Cube;

use crate::math::{Vec3, Ray};
use crate::material::Material;

// Registro de información de colisión de un rayo con un objeto
#[derive(Debug, Clone)]
pub struct HitRecord {
    // Punto de intersección
    pub point: Vec3,
    // Normal de la superficie en el punto de intersección
    pub normal: Vec3,
    // Parámetro t del rayo donde ocurrió la intersección
    pub t: f32,
    // Indica si el rayo golpeó desde el frente (true) o desde atrás (false)
    pub front_face: bool,
    // Material del objeto golpeado
    pub material: Material,
    // Coordenadas UV para mapeo de texturas (u=horizontal, v=vertical)
    pub u: f32,
    pub v: f32,
}

impl HitRecord {
    /// Crea un nuevo HitRecord
    pub fn new(point: Vec3, normal: Vec3, t: f32, ray: &Ray, material: Material, u: f32, v: f32) -> Self {
        let front_face = ray.direction.dot(&normal) < 0.0;
        let normal = if front_face { normal } else { -normal };
        
        HitRecord {
            point,
            normal,
            t,
            front_face,
            material,
            u,
            v,
        }
    }
}

pub trait Hittable: Send + Sync + std::fmt::Debug {
    // Verifica si el rayo intersecta el objeto entre t_min y t_max
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

// Lista de objetos que pueden ser intersectados (thread-safe)
pub struct HittableList {
    objects: Vec<Box<dyn Hittable + Send + Sync>>,
}

// Implementar Debug manualmente para HittableList
impl std::fmt::Debug for HittableList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HittableList")
            .field("objects", &format!("{} objects", self.objects.len()))
            .finish()
    }
}

// Implementar Send + Sync manualmente para HittableList
unsafe impl Send for HittableList {}
unsafe impl Sync for HittableList {}

impl HittableList {
    pub fn new() -> Self {
        HittableList {
            objects: Vec::new(),
        }
    }

    pub fn add<T: Hittable + Send + Sync + std::fmt::Debug + 'static>(&mut self, object: T) {
        self.objects.push(Box::new(object));
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest_hit: Option<HitRecord> = None;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if let Some(hit_record) = object.hit(ray, t_min, closest_so_far) {
                closest_so_far = hit_record.t;
                closest_hit = Some(hit_record);
            }
        }

        closest_hit
    }
}
