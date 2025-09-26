// Motor de raytracing

use crate::math::{Vec3, Ray};
use crate::scene::Scene;
use crate::camera::Camera;
use crate::geometry::HitRecord;
use raylib::prelude::*;

pub struct Raytracer {
    pub width: u32,
    pub height: u32,
    pub max_depth: i32,
}

impl Raytracer {
    // Crea un nuevo raytracer
    pub fn new(width: u32, height: u32) -> Self {
        Raytracer {
            width,
            height,
            max_depth: 5,
        }
    }
    
    // Renderiza la escena completa
    pub fn render(&self, scene: &Scene, camera: &Camera) -> Vec<Vec<Color>> {
        let mut image = vec![vec![Color::BLACK; self.width as usize]; self.height as usize];
        
        println!("Renderizando {}x{} pixels", self.width, self.height);
        
        for y in 0..self.height {
            // Mostrar progreso cada 50 líneas para menos overhead
            if y % 50 == 0 {
                let progress = (y as f32 / self.height as f32 * 100.0) as u32;
                print!("\r{}%", progress);
                use std::io::{self, Write};
                io::stdout().flush().unwrap();
            }
            for x in 0..self.width {
                // Convierte coordenadas de pixel a coordenadas UV [0,1]
                let u = x as f32 / (self.width - 1) as f32;
                let v = (self.height - 1 - y) as f32 / (self.height - 1) as f32; // Invertir Y
                
                // Genera el rayo para este pixel
                let ray = camera.get_ray(u, v);
                
                // Calcula el color del pixel
                let color = self.ray_color(&ray, scene, self.max_depth);
                
                // Convierte el color a formato Raylib
                image[y as usize][x as usize] = vec3_to_color(color);
            }
        }
        
        println!("\nRenderizado completo!");
        
        println!("Renderizado completo!");
        image
    }
    
    // Calcula el color que debe tener un rayo
    fn ray_color(&self, ray: &Ray, scene: &Scene, depth: i32) -> Vec3 {
        // Si hemos alcanzado el límite de rebotes, no contribuye más luz
        if depth <= 0 {
            return Vec3::zero();
        }
        
        // Verifica si el rayo golpea algún objeto
        if let Some(hit_record) = scene.hit(ray, 0.001, f32::INFINITY) {
            self.calculate_lighting(&hit_record, ray, scene, depth)
        } else {
            // Si no golpea nada, devuelve el color de fondo
            scene.get_background_color(ray)
        }
    }
    
    // Calcula la iluminación en un punto de intersección
    fn calculate_lighting(&self, hit: &HitRecord, incident_ray: &Ray, scene: &Scene, depth: i32) -> Vec3 {
        let mut color = Vec3::zero();
        
        // Emisión del material (si es emisivo)
        color += hit.material.emitted();
        
        // Luz ambiental
        let surface_color = hit.material.texture.value(hit.u, hit.v);
        color += scene.ambient_light * surface_color;
        
        // Contribución de todas las luces
        for (light, shadow_factor) in scene.get_lights_affecting_point(hit.point) {
            let light_dir = light.get_direction_from(hit.point);
            let light_color = light.get_effective_color(hit.point);
            
            // Componente difusa (Lambertian)
            let diffuse_strength = hit.normal.dot(&light_dir).max(0.0);
            let diffuse = surface_color * light_color * diffuse_strength * shadow_factor;
            color += diffuse;
            
            // Componente especular (Phong/Blinn-Phong)
            if hit.material.specular > 0.0 && diffuse_strength > 0.0 {
                let view_dir = (-incident_ray.direction).normalize();
                let reflect_dir = (-light_dir).reflect(&hit.normal);
                
                let spec_strength = view_dir.dot(&reflect_dir).max(0.0)
                    .powf((1.0 - hit.material.roughness) * 128.0);
                
                let specular = light_color * hit.material.specular * spec_strength * shadow_factor;
                color += specular;
            }
        }
        
        // Reflexión
        if hit.material.reflectivity > 0.0 && depth > 1 {
            let reflected = incident_ray.direction.reflect(&hit.normal);
            let reflection_ray = Ray::new(hit.point + hit.normal * 0.001, reflected);
            let reflection_color = self.ray_color(&reflection_ray, scene, depth - 1);
            color += reflection_color * hit.material.reflectivity;
        }
        
        // Refracción
        if hit.material.transparency > 0.0 && depth > 1 {
            let refraction_ratio = if hit.front_face {
                1.0 / hit.material.refractive_index
            } else {
                hit.material.refractive_index
            };
            
            if let Some(refracted) = incident_ray.direction.refract(&hit.normal, refraction_ratio) {
                let refraction_ray = Ray::new(hit.point - hit.normal * 0.001, refracted);
                let refraction_color = self.ray_color(&refraction_ray, scene, depth - 1);
                
                // Mejor balance entre reflexión y refracción según el ángulo de Fresnel
                let view_dir = (-incident_ray.direction).normalize();
                let cos_theta = view_dir.dot(&hit.normal).abs();
                let fresnel = 0.04 + (1.0 - 0.04) * (1.0 - cos_theta).powf(5.0); // Aproximación Schlick
                
                // Mezclar reflexión y refracción según Fresnel
                let refraction_strength = hit.material.transparency * (1.0 - fresnel);
                color += refraction_color * refraction_strength;
                
                // Solo reducir la componente difusa, no todo el color
                let surface_component = surface_color * scene.ambient_light;
                color = color - surface_component * hit.material.transparency + surface_component * (1.0 - hit.material.transparency);
            }
        }
        
        // Clamp el color a [0,1]
        color.clamp(0.0, 1.0)
    }
}

// Convierte un Vec3 a Color de Raylib
fn vec3_to_color(color: Vec3) -> Color {
    Color::new(
        (color.x.clamp(0.0, 1.0) * 255.0) as u8,
        (color.y.clamp(0.0, 1.0) * 255.0) as u8,
        (color.z.clamp(0.0, 1.0) * 255.0) as u8,
        255,
    )
}
