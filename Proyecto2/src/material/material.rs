// Sistema de materiales para raytracing

use crate::math::Vec3;

#[derive(Debug, Clone)]
pub struct Material {
    // Color base del material (albedo)
    pub color: Vec3,
    // Componente especular (0.0 = mate, 1.0 = muy especular)
    pub specular: f32,
    // Rugosidad del material (0.0 = espejo perfecto, 1.0 = completamente rugoso)
    pub roughness: f32,
    // Reflectividad (0.0 = no reflectivo, 1.0 = completamente reflectivo)
    pub reflectivity: f32,
    // Transparencia (0.0 = opaco, 1.0 = completamente transparente)
    pub transparency: f32,
    // Índice de refracción (usado cuando transparency > 0)
    pub refractive_index: f32,
    // Factor de emisión (para materiales que emiten luz)
    pub emission: Vec3,
}

impl Material {
    // Crea un nuevo material con valores por defecto
    pub fn new() -> Self {
        Material {
            color: Vec3::new(0.7, 0.7, 0.7),  
            specular: 0.1,
            roughness: 0.8,
            reflectivity: 0.0,
            transparency: 0.0,
            refractive_index: 1.0,
            emission: Vec3::zero(),
        }
    }
    
    // Builder pattern para configurar el color
    pub fn with_color(mut self, color: Vec3) -> Self {
        self.color = color;
        self
    }
    
    // Builder pattern para configurar especularidad
    pub fn with_specular(mut self, specular: f32) -> Self {
        self.specular = specular.clamp(0.0, 1.0);
        self
    }
    
    // Builder pattern para configurar rugosidad
    pub fn with_roughness(mut self, roughness: f32) -> Self {
        self.roughness = roughness.clamp(0.0, 1.0);
        self
    }
    
    // Builder pattern para configurar reflectividad
    pub fn with_reflectivity(mut self, reflectivity: f32) -> Self {
        self.reflectivity = reflectivity.clamp(0.0, 1.0);
        self
    }
    
    // Builder pattern para configurar transparencia
    pub fn with_transparency(mut self, transparency: f32) -> Self {
        self.transparency = transparency.clamp(0.0, 1.0);
        self
    }
    
    // Builder pattern para configurar índice de refracción
    pub fn with_refractive_index(mut self, refractive_index: f32) -> Self {
        self.refractive_index = refractive_index.max(1.0);
        self
    }
    
    // Builder pattern para configurar emisión
    pub fn with_emission(mut self, emission: Vec3) -> Self {
        self.emission = emission;
        self
    }
    
    // Verifica si el material es emisivo
    pub fn is_emissive(&self) -> bool {
        self.emission.length_squared() > 0.0
    }
    
    // Obtiene el color de emisión
    pub fn emitted(&self) -> Vec3 {
        self.emission
    }
}
