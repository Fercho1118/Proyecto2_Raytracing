// Gestión de escena para raytracing

use crate::math::{Vec3, Ray};
use crate::geometry::{Cube, HittableList, HitRecord, Hittable};
use crate::lighting::Light;

const EPSILON: f32 = 0.001;

#[derive(Debug)]
pub struct Scene {
    // Lista de todos los objetos en la escena
    pub objects: HittableList,
    // Lista de luces en la escena
    pub lights: Vec<Light>,
    // Color de fondo de la escena
    pub background_color: Vec3,
    // Luz ambiental global
    pub ambient_light: Vec3,
}

impl Scene {
    // Crea una nueva escena vacía
    pub fn new() -> Self {
        Scene {
            objects: HittableList::new(),
            lights: Vec::new(),
            background_color: Vec3::new(0.1, 0.1, 0.2), 
            ambient_light: Vec3::new(0.1, 0.1, 0.1),     
        }
    }
    
    // Establece el color de fondo
    pub fn set_background_color(&mut self, color: Vec3) {
        self.background_color = color;
    }
    
    // Añade un cubo a la escena
    pub fn add_cube(&mut self, cube: Cube) {
        self.objects.add(cube);
    }
    
    // Añade una luz a la escena
    pub fn add_light(&mut self, light: Light) {
        self.lights.push(light);
    }
    
    // Verifica si un rayo intersecta algún objeto de la escena
    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self.objects.hit(ray, t_min, t_max)
    }
    
    // Obtiene el color del fondo con skybox procedural
    pub fn get_background_color(&self, ray: &Ray) -> Vec3 {
        self.procedural_skybox(ray.direction.normalize())
    }
    
    // Skybox procedural completo - simula estar dentro de un cubo de cielo
    fn procedural_skybox(&self, direction: Vec3) -> Vec3 {
        let dir = direction.normalize();
        
        // === CONFIGURACIÓN DEL SKYBOX ===
        let sun_position = Vec3::new(0.4, 0.6, -0.3).normalize(); // Sol en posición diagonal
        
        // Colores base para diferentes direcciones
        let sky_blue_top = Vec3::new(0.3, 0.6, 1.0);      
        let sky_blue_horizon = Vec3::new(0.7, 0.85, 1.0); 
        let sunset_orange = Vec3::new(1.0, 0.7, 0.4);     
        let ground_sky = Vec3::new(0.4, 0.7, 0.9);        
        
        // === DETERMINAR EL COLOR BASE SEGÚN DIRECCIÓN ===
        let mut base_color = if dir.y > 0.3 {
            // Parte superior del cielo
            let t = ((dir.y - 0.3) / 0.7).powf(0.8);
            sky_blue_horizon.lerp(&sky_blue_top, t)
        } else if dir.y > -0.3 {
            // Zona del horizonte
            let sunset_influence = (1.0 - dir.y.abs()).powf(2.0);
            let base = sky_blue_horizon;
            base.lerp(&sunset_orange, sunset_influence * 0.3)
        } else {
            // Parte inferior 
            let t = ((-dir.y - 0.3) / 0.7).powf(0.5);
            sky_blue_horizon.lerp(&ground_sky, t * 0.6)
        };
        
        // === SOL CON EFECTOS REALISTAS ===
        let sun_angle = dir.dot(&sun_position).max(0.0);
        
        if sun_angle > 0.998 {
            // Núcleo del sol - muy brillante
            base_color = Vec3::new(1.0, 1.0, 0.9);
        } else if sun_angle > 0.995 {
            // Halo interior
            let intensity = (sun_angle - 0.995) / (0.998 - 0.995);
            let sun_core = Vec3::new(1.0, 0.95, 0.7);
            base_color = base_color.lerp(&sun_core, intensity);
        } else if sun_angle > 0.99 {
            // Halo medio
            let intensity = (sun_angle - 0.99) / (0.995 - 0.99);
            let sun_glow = Vec3::new(1.0, 0.8, 0.5);
            base_color = base_color.lerp(&sun_glow, intensity * 0.8);
        } else if sun_angle > 0.96 {
            // Resplandor exterior
            let intensity = (sun_angle - 0.96) / (0.99 - 0.96);
            let sun_outer = Vec3::new(1.0, 0.7, 0.4);
            base_color = base_color.lerp(&sun_outer, intensity * 0.4);
        }
        
        // === NUBES PROCEDURALES MEJORADAS ===
        if dir.y > -0.5 { 
            let cloud_pos1 = dir * 8.0;
            let cloud_pos2 = dir * 16.0;
            let cloud_pos3 = dir * 32.0;
            
            let noise1 = self.simple_noise(cloud_pos1.x + cloud_pos1.z, cloud_pos1.y) * 0.6;
            let noise2 = self.simple_noise(cloud_pos2.x - cloud_pos2.z, cloud_pos2.y + 100.0) * 0.3;
            let noise3 = self.simple_noise(cloud_pos3.x, cloud_pos3.z + 200.0) * 0.1;
            
            let cloud_noise = (noise1 + noise2 + noise3 + 0.5).clamp(0.0, 1.0);
            let cloud_threshold = 0.6;
            
            if cloud_noise > cloud_threshold {
                let cloud_density = ((cloud_noise - cloud_threshold) / (1.0 - cloud_threshold)).powf(1.5);
                
                // Color de nube que depende de la posición del sol
                let sun_influence = (dir.dot(&sun_position) + 1.0) * 0.5;
                let cloud_base = Vec3::new(0.9, 0.9, 0.95);
                let cloud_lit = Vec3::new(1.0, 0.95, 0.85);
                let cloud_shadow = Vec3::new(0.6, 0.65, 0.7);
                
                let cloud_color = if sun_influence > 0.7 {
                    cloud_base.lerp(&cloud_lit, (sun_influence - 0.7) * 3.0)
                } else {
                    cloud_base.lerp(&cloud_shadow, (0.7 - sun_influence) * 0.5)
                };
                
                // Mezclar con intensidad variable
                let altitude_factor = (dir.y + 1.0) * 0.5; // Más nubes en el horizonte
                let cloud_alpha = cloud_density * (0.4 + 0.6 * (1.0 - altitude_factor));
                
                base_color = base_color.lerp(&cloud_color, cloud_alpha);
            }
        }
        
        // === EFECTOS ATMOSFÉRICOS ===
        // Ligero tinte cálido cerca del horizonte
        if dir.y.abs() < 0.4 {
            let horizon_warmth = Vec3::new(1.0, 0.9, 0.7);
            let warmth_factor = (0.4 - dir.y.abs()) / 0.4 * 0.1;
            base_color = base_color.lerp(&horizon_warmth, warmth_factor);
        }
        
        base_color.clamp(0.0, 1.0)
    }
    
    // Generador de ruido simple para efectos procedurales
    fn simple_noise(&self, x: f32, y: f32) -> f32 {
        let n = (x * 57.0 + y * 131.0).sin() * 43758.5453;
        (n - n.floor()) * 2.0 - 1.0
    }
    
    // Verifica si hay una línea de vista clara entre dos puntos (para sombras)
    pub fn is_in_shadow(&self, from: Vec3, to: Vec3) -> bool {
        let direction = to - from;
        let distance = direction.length();
        let ray = Ray::new(from, direction.normalize());
        
        // Verifica si hay algún objeto entre los dos puntos
        if let Some(hit) = self.hit(&ray, EPSILON, distance - EPSILON) {
            // Si el material del objeto golpeado es transparente, no genera sombra completa
            hit.material.transparency < 0.9
        } else {
            false
        }
    }
    
    // Obtiene todas las luces que afectan un punto (excluyendo las que están en sombra)
    pub fn get_lights_affecting_point(&self, point: Vec3) -> Vec<(&Light, f32)> {
        let mut affecting_lights = Vec::new();
        
        for light in &self.lights {
            // Para luces puntuales, verifica si hay sombra
            let shadow_factor = if self.is_in_shadow(point, light.position) {
                0.3 // Sombra parcial
            } else {
                1.0 // Sin sombra
            };
            
            affecting_lights.push((light, shadow_factor));
        }
        
        affecting_lights
    }
}