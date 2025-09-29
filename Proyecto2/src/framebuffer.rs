use crate::math::Vec3;

#[derive(Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8, 
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn from_vec3(v: &Vec3) -> Self {
        Self {
            r: (v.x.clamp(0.0, 1.0) * 255.0) as u8,
            g: (v.y.clamp(0.0, 1.0) * 255.0) as u8,
            b: (v.z.clamp(0.0, 1.0) * 255.0) as u8,
            a: 255,
        }
    }
    
    pub fn to_u32(&self) -> u32 {
        ((self.a as u32) << 24) | 
        ((self.r as u32) << 16) | 
        ((self.g as u32) << 8) | 
        (self.b as u32)
    }
}

pub struct Framebuffer {
    pub width: u32,
    pub height: u32,
    pixels: Vec<u32>,
}

impl Framebuffer {
    pub fn new(width: u32, height: u32) -> Self {
        Self { 
            width, 
            height, 
            pixels: vec![0; (width * height) as usize] 
        }
    }
    
    pub fn set_pixel(&mut self, x: u32, y: u32, color: Color) {
        if x < self.width && y < self.height {
            let idx = (y * self.width + x) as usize;
            self.pixels[idx] = color.to_u32();
        }
    }
    
    pub fn set_pixel_from_vec3(&mut self, x: u32, y: u32, color: Vec3) {
        self.set_pixel(x, y, Color::from_vec3(&color));
    }
    
    // Para compatibilidad con raylib, convertir a Vec<Vec<raylib::Color>>
    pub fn to_raylib_buffer(&self) -> Vec<Vec<raylib::prelude::Color>> {
        let mut buffer = Vec::with_capacity(self.height as usize);
        
        for y in 0..self.height {
            let mut row = Vec::with_capacity(self.width as usize);
            for x in 0..self.width {
                let idx = (y * self.width + x) as usize;
                let pixel = self.pixels[idx];
                let color = raylib::prelude::Color::new(
                    ((pixel >> 16) & 0xFF) as u8,
                    ((pixel >> 8) & 0xFF) as u8,
                    (pixel & 0xFF) as u8,
                    ((pixel >> 24) & 0xFF) as u8,
                );
                row.push(color);
            }
            buffer.push(row);
        }
        
        buffer
    }
}