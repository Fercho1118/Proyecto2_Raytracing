// Motor de raytracing optimizado con paralelización

use crate::math::{Vec3, Ray};
use crate::scene::Scene;
use crate::camera::Camera;
use crate::geometry::HitRecord;
use crate::framebuffer::Framebuffer;
use crate::adaptive_config::{RenderQuality, AdaptiveConfig};
use rayon::prelude::*;

pub struct Raytracer {
    pub width: u32,
    pub height: u32,
    pub max_depth: i32,
    pub quality: RenderQuality,
}

impl Raytracer {
    // Crea un raytracer con configuración específica
    pub fn with_config(config: &AdaptiveConfig) -> Self {
        let (width, height) = config.quality.dimensions();
        Raytracer {
            width,
            height,
            max_depth: config.quality.max_depth(),
            quality: config.quality,
        }
    }
    
    // Actualiza la calidad dinámicamente
    pub fn set_quality(&mut self, quality: RenderQuality) {
        self.quality = quality;
        let (width, height) = quality.dimensions();
        self.width = width;
        self.height = height;
        self.max_depth = quality.max_depth();
    }
    
    // Método para renderizado directo a framebuffer (más eficiente)
    pub fn render_to_framebuffer(&self, scene: &Scene, camera: &Camera, framebuffer: &mut Framebuffer) {
        println!("Renderizando {}x{} pixels directamente a framebuffer...", self.width, self.height);
        
        let total_pixels = (self.width * self.height) as usize;
        let mut pixel_data: Vec<Vec3> = vec![Vec3::zero(); total_pixels];
        
        // Renderizado paralelo
        pixel_data
            .par_iter_mut()
            .enumerate()
            .for_each(|(idx, pixel_color)| {
                let y = idx / (self.width as usize);
                let x = idx % (self.width as usize);
                
                let u = x as f32 / (self.width - 1) as f32;
                let v = (self.height - 1 - y as u32) as f32 / (self.height - 1) as f32;
                
                let ray = camera.get_ray(u, v);
                *pixel_color = self.ray_color(&ray, scene, self.max_depth);
                
                if idx % 20000 == 0 {
                    let progress = (idx as f32 / total_pixels as f32 * 100.0) as u32;
                    print!("\r{}%", progress);
                    use std::io::{self, Write};
                    let _ = io::stdout().flush();
                }
            });
        
        // Escribir al framebuffer
        for (idx, &color) in pixel_data.iter().enumerate() {
            let y = (idx / (self.width as usize)) as u32;
            let x = (idx % (self.width as usize)) as u32;
            framebuffer.set_pixel_from_vec3(x, y, color);
        }
        
        println!("\nRenderizado directo completo!");
    }
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
    
        // Calcula el color que debe tener un rayo (optimizado)
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
