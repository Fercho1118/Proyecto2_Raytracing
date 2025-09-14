// Definición de texturas

use crate::math::Vec3;

// Datos de imagen cargada
#[derive(Debug, Clone)]
pub struct ImageData {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<Vec3>, // RGB convertido a Vec3
}

#[derive(Debug, Clone)]
pub enum Texture {
    // Color sólido
    SolidColor { color: Vec3 },
    
    // Textura de imagen cargada desde archivo
    ImageTexture {
        image_data: ImageData,
    },
}

impl Texture {
    // Crea una textura de color sólido
    pub fn solid_color(color: Vec3) -> Self {
        Texture::SolidColor { color }
    }
    
    // Carga una textura desde archivo de imagen
    pub fn from_file(file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        use image::ImageReader;
        
        let img = ImageReader::open(file_path)?.decode()?;
        let rgb_img = img.to_rgb8();
        
        let width = rgb_img.width();
        let height = rgb_img.height();
        
        let mut pixels = Vec::with_capacity((width * height) as usize);
        
        for pixel in rgb_img.pixels() {
            let r = pixel[0] as f32 / 255.0;
            let g = pixel[1] as f32 / 255.0;
            let b = pixel[2] as f32 / 255.0;
            pixels.push(Vec3::new(r, g, b));
        }
        
        let image_data = ImageData {
            width,
            height,
            pixels,
        };
        
        Ok(Texture::ImageTexture { image_data })
    }
    
    // Obtiene el color de la textura en las coordenadas UV dadas
    pub fn value(&self, u: f32, v: f32) -> Vec3 {
        match self {
            Texture::SolidColor { color } => *color,
            
            Texture::ImageTexture { image_data } => {
                // Convertir coordenadas UV [0,1] a coordenadas de pixel
                let x = (u * image_data.width as f32).floor() as u32;
                let y = (v * image_data.height as f32).floor() as u32;
                
                // Asegurar que estamos dentro de los límites
                let x = x.min(image_data.width - 1);
                let y = y.min(image_data.height - 1);
                
                // Obtener el pixel 
                let y_flipped = image_data.height - 1 - y;
                let pixel_index = (y_flipped * image_data.width + x) as usize;
                
                image_data.pixels[pixel_index]
            }
        }
    }
}
