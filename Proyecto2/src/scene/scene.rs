// Gestión de escena para raytracing

use crate::math::{Vec3, Ray};
use crate::geometry::{Plane, Cube, HittableList, HitRecord, Hittable};
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
    
    // Añade un plano a la escena
    pub fn add_plane(&mut self, plane: Plane) {
        self.objects.add(plane);
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
    
    // Obtiene el color del fondo en una dirección dada
    pub fn get_background_color(&self, _ray: &Ray) -> Vec3 {
        // Por ahora retorna un color sólido
        // Más tarde podemos implementar un skybox aquí
        self.background_color
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