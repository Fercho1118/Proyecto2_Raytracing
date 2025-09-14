// Sistema de cámara para raytracing

use crate::math::{Vec3, Ray};

#[derive(Debug, Clone)]
pub struct Camera {
    // Posición de la cámara en el espacio
    pub position: Vec3,
    // Punto hacia donde mira la cámara
    pub target: Vec3,
    // Vector "arriba" de la cámara
    pub up: Vec3,
    // Campo de visión vertical en grados
    pub fov: f32,
    // Proporción de aspecto 
    pub aspect_ratio: f32,
    
    // Vectores calculados del sistema de coordenadas de la cámara
    pub forward: Vec3,
    pub right: Vec3,
    pub camera_up: Vec3,
    
    // Parámetros del plano de imagen
    pub viewport_height: f32,
    pub viewport_width: f32,
    pub focal_length: f32,
    
    // Esquinas del viewport
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
}

impl Camera {
    // Crea una nueva cámara
    pub fn new(position: Vec3, target: Vec3, up: Vec3, fov: f32, aspect_ratio: f32) -> Self {
        let mut camera = Camera {
            position,
            target,
            up,
            fov,
            aspect_ratio,
            forward: Vec3::zero(),
            right: Vec3::zero(),
            camera_up: Vec3::zero(),
            viewport_height: 0.0,
            viewport_width: 0.0,
            focal_length: 1.0,
            lower_left_corner: Vec3::zero(),
            horizontal: Vec3::zero(),
            vertical: Vec3::zero(),
        };
        
        camera.update_camera_vectors();
        camera.update_viewport();
        camera
    }
    
    // Actualiza los vectores del sistema de coordenadas de la cámara
    fn update_camera_vectors(&mut self) {
        // Calcular el vector forward (hacia donde mira la cámara)
        self.forward = (self.target - self.position).normalize();
        
        // Calcular el vector right (hacia la derecha de la cámara)
        self.right = self.forward.cross(&self.up).normalize();
        
        // Calcular el vector up de la cámara (perpendicular a forward y right)
        self.camera_up = self.right.cross(&self.forward).normalize();
    }
    
    // Actualiza los parámetros del viewport
    fn update_viewport(&mut self) {
        let theta = crate::math::degrees_to_radians(self.fov);
        let half_height = (theta / 2.0).tan();
        
        self.viewport_height = 2.0 * half_height;
        self.viewport_width = self.aspect_ratio * self.viewport_height;
        
        self.horizontal = self.right * self.viewport_width;
        self.vertical = self.camera_up * self.viewport_height;
        
        self.lower_left_corner = self.position - self.horizontal * 0.5 
                               - self.vertical * 0.5 - self.forward * self.focal_length;
    }
    
    // Genera un rayo desde la cámara hacia las coordenadas (u, v) del viewport
    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        // Convierte u,v [0,1] a coordenadas NDC [-1,1]
        let ndc_x = (u * 2.0) - 1.0;
        let ndc_y = (v * 2.0) - 1.0;
        
        // Calcula el punto en el plano de la imagen
        let half_width = (crate::math::degrees_to_radians(self.fov) * 0.5).tan() * self.aspect_ratio;
        let half_height = (crate::math::degrees_to_radians(self.fov) * 0.5).tan();
        
        let target_point = self.position + self.forward * self.focal_length
                          + self.right * (ndc_x * half_width)
                          + self.camera_up * (ndc_y * half_height);
        
        let direction = (target_point - self.position).normalize();
        Ray::new(self.position, direction)
    }
    
    // Controles de cámara interactivos
    
    /// Rota la cámara alrededor del target usando delta del mouse
    pub fn rotate_around_target(&mut self, delta_x: f32, delta_y: f32, sensitivity: f32) {
        let distance = (self.position - self.target).length();
        
        // Calcular ángulos esféricos actuales
        let direction = (self.position - self.target).normalize();
        let theta = direction.z.atan2(direction.x); // Ángulo horizontal
        let phi = direction.y.asin().clamp(-1.5, 1.5); // Ángulo vertical (limitar)
        
        // Aplicar rotación
        let new_theta = theta - delta_x * sensitivity;
        let new_phi = (phi + delta_y * sensitivity).clamp(-1.5, 1.5);
        
        // Convertir de vuelta a coordenadas cartesianas
        let new_direction = Vec3::new(
            new_phi.cos() * new_theta.cos(),
            new_phi.sin(),
            new_phi.cos() * new_theta.sin()
        );
        
        // Nueva posición
        self.position = self.target + new_direction * distance;
        self.update_camera_vectors();
    }
    
    /// Zoom (cambiar distancia al target)
    pub fn zoom(&mut self, delta: f32, speed: f32) {
        let direction = (self.position - self.target).normalize();
        let current_distance = (self.position - self.target).length();
        let new_distance = (current_distance - delta * speed).max(0.1); // Mínimo 0.1
        
        self.position = self.target + direction * new_distance;
        self.update_camera_vectors();
    }
}
