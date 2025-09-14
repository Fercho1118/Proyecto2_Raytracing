// Sistema de luces para raytracing

use crate::math::Vec3;

#[derive(Debug, Clone)]
pub struct Light {
    // Posición de la luz en el espacio
    pub position: Vec3,
    // Color de la luz
    pub color: Vec3,
    // Intensidad de la luz 
    pub intensity: f32,
}

impl Light {
    // Crea una nueva luz puntual
    pub fn new(position: Vec3, color: Vec3, intensity: f32) -> Self {
        Light {
            position,
            color,
            intensity,
        }
    }
    
    // Obtiene la dirección de la luz desde un punto dado
    pub fn get_direction_from(&self, point: Vec3) -> Vec3 {
        (self.position - point).normalize()
    }
    
    // Obtiene la distancia a la luz desde un punto (para atenuación)
    pub fn get_distance_from(&self, point: Vec3) -> f32 {
        (self.position - point).length()
    }
    
    // Calcula la atenuación de la luz basada en la distancia
    pub fn get_attenuation(&self, distance: f32) -> f32 {
        // Atenuación cuadrática para luces puntuales
        let constant = 1.0;
        let linear = 0.09;
        let quadratic = 0.032;
        1.0 / (constant + linear * distance + quadratic * (distance * distance))
    }
    
    // Obtiene el color efectivo de la luz considerando intensidad y atenuación
    pub fn get_effective_color(&self, point: Vec3) -> Vec3 {
        let distance = self.get_distance_from(point);
        let attenuation = self.get_attenuation(distance);
        self.color * self.intensity * attenuation
    }
}
