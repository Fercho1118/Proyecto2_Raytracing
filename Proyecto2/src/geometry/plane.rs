// Plano para raytracing

use crate::math::{Vec3, Ray};
use crate::material::Material;
use super::{HitRecord, Hittable};

const EPSILON: f32 = 0.001;

#[derive(Debug, Clone)]
pub struct Plane {
    pub point: Vec3,    // Un punto en el plano
    pub normal: Vec3,   // Normal del plano 
    pub material: Material,
}

impl Plane {
    pub fn new(point: Vec3, normal: Vec3, material: Material) -> Self {
        Plane {
            point,
            normal: normal.normalize(),
            material,
        }
    }
}

impl Hittable for Plane {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let denom = self.normal.dot(&ray.direction);
        
        // Si el denominador es muy pequeño, el rayo es paralelo al plano
        if denom.abs() < EPSILON {
            return None;
        }
        
        let t = (self.point - ray.origin).dot(&self.normal) / denom;
        
        // Verifica que t esté en el rango válido
        if t < t_min || t > t_max {
            return None;
        }
        
        let point = ray.at(t);
        
        // Coordenadas UV básicas para el plano (repetir cada 2 unidades)
        let u = (point.x * 0.5) % 1.0;
        let v = (point.z * 0.5) % 1.0;
        let u = if u < 0.0 { u + 1.0 } else { u };
        let v = if v < 0.0 { v + 1.0 } else { v };
        
        Some(HitRecord::new(point, self.normal, t, ray, self.material.clone(), u, v))
    }
}
