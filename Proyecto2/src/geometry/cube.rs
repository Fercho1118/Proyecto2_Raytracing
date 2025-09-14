// Cubos para raytracing

use crate::math::{Vec3, Ray};
use crate::material::Material;
use super::{HitRecord, Hittable};

const EPSILON: f32 = 0.001;

#[derive(Debug, Clone)]
pub struct Cube {
    pub min: Vec3,      
    pub max: Vec3,      
    pub material: Material,
}

impl Cube {
    // Crea un nuevo cubo desde el centro y el tamaño
    pub fn new(center: Vec3, size: Vec3, material: Material) -> Self {
        let half_size = size * 0.5;
        Cube {
            min: center - half_size,
            max: center + half_size,
            material,
        }
    }
}

impl Hittable for Cube {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        // Algoritmo de intersección 
        
        let mut t_min = t_min;
        let mut t_max = t_max;
        
        // Variables para rastrear qué cara fue golpeada
        let mut hit_normal = Vec3::new(0.0, 0.0, 0.0);
        
        // Verifica cada par de planos (x, y, z)
        for axis in 0..3 {
            let ray_dir = match axis {
                0 => ray.direction.x,
                1 => ray.direction.y,
                _ => ray.direction.z,
            };
            
            let ray_origin = match axis {
                0 => ray.origin.x,
                1 => ray.origin.y,
                _ => ray.origin.z,
            };
            
            let min_val = match axis {
                0 => self.min.x,
                1 => self.min.y,
                _ => self.min.z,
            };
            
            let max_val = match axis {
                0 => self.max.x,
                1 => self.max.y,
                _ => self.max.z,
            };
            
            let inv_dir = 1.0 / ray_dir;
            let mut t0 = (min_val - ray_origin) * inv_dir;
            let mut t1 = (max_val - ray_origin) * inv_dir;
            
            if inv_dir < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }
            
            if t0 > t_min {
                t_min = t0;
                // Determina la normal basada en qué plano fue golpeado
                hit_normal = Vec3::new(0.0, 0.0, 0.0);
                match axis {
                    0 => hit_normal.x = if inv_dir < 0.0 { 1.0 } else { -1.0 },
                    1 => hit_normal.y = if inv_dir < 0.0 { 1.0 } else { -1.0 },
                    _ => hit_normal.z = if inv_dir < 0.0 { 1.0 } else { -1.0 },
                }
            }
            
            if t1 < t_max {
                t_max = t1;
            }
            
            if t_max < t_min {
                return None; // No hay intersección
            }
        }
        
        // Si se llega aquí, hay una intersección
        let t = if t_min > EPSILON { t_min } else { t_max };
        
        if t < EPSILON {
            return None; // Intersección detrás del origen del rayo
        }
        
        let point = ray.at(t);
        
        if hit_normal.length_squared() == 0.0 {
            // Determina qué cara del cubo está más cerca del punto de intersección
            let center = (self.min + self.max) * 0.5;
            let local_point = point - center;
            let half_size = (self.max - self.min) * 0.5;
            
            // Encuentra el componente con mayor valor absoluto relativo
            let abs_x = (local_point.x / half_size.x).abs();
            let abs_y = (local_point.y / half_size.y).abs();
            let abs_z = (local_point.z / half_size.z).abs();
            
            if abs_x >= abs_y && abs_x >= abs_z {
                hit_normal = Vec3::new(if local_point.x > 0.0 { 1.0 } else { -1.0 }, 0.0, 0.0);
            } else if abs_y >= abs_x && abs_y >= abs_z {
                hit_normal = Vec3::new(0.0, if local_point.y > 0.0 { 1.0 } else { -1.0 }, 0.0);
            } else {
                hit_normal = Vec3::new(0.0, 0.0, if local_point.z > 0.0 { 1.0 } else { -1.0 });
            }
        }
        
        // Calcula las coordenadas UV según la cara golpeada
        let center = (self.min + self.max) * 0.5;
        let local_point = point - center;
        let half_size = (self.max - self.min) * 0.5;
        
        let (u, v) = if hit_normal.x.abs() > 0.5 {
            // Cara X (izquierda o derecha)
            let u = (local_point.z / half_size.z + 1.0) * 0.5;
            let v = (local_point.y / half_size.y + 1.0) * 0.5;
            (u, 1.0 - v)
        } else if hit_normal.y.abs() > 0.5 {
            // Cara Y (arriba o abajo) 
            let u = (local_point.x / half_size.x + 1.0) * 0.5;
            let v = (local_point.z / half_size.z + 1.0) * 0.5;
            (u, 1.0 - v)
        } else {
            // Cara Z (frente o atrás)
            let u = (local_point.x / half_size.x + 1.0) * 0.5;
            let v = (local_point.y / half_size.y + 1.0) * 0.5;
            (u, 1.0 - v)
        };
        
        Some(HitRecord::new(point, hit_normal, t, ray, self.material.clone(), u, v))
    }
}
