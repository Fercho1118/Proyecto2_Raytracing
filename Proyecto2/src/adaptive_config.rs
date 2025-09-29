#[derive(Clone, Copy, Debug)]
pub enum RenderQuality {
    Low,      // 400x300
    Medium,   // 600x450  
    High,     // 800x600
}

impl RenderQuality {
    pub fn dimensions(&self) -> (u32, u32) {
        match self {
            RenderQuality::Low => (400, 300),
            RenderQuality::Medium => (600, 450),
            RenderQuality::High => (800, 600),
        }
    }
    
    pub fn max_depth(&self) -> i32 {
        match self {
            RenderQuality::Low => 3,
            RenderQuality::Medium => 4,
            RenderQuality::High => 5,
        }
    }
    
    pub fn description(&self) -> &str {
        match self {
            RenderQuality::Low => "Baja (400x300)",
            RenderQuality::Medium => "Media (600x450)",
            RenderQuality::High => "Alta (800x600)",
        }
    }
}

// Configuración del renderizador adaptativo
#[derive(Clone)]
pub struct AdaptiveConfig {
    pub quality: RenderQuality,
    pub enable_auto_rotation: bool,
    pub rotation_speed: f32,
}

impl AdaptiveConfig {
    pub fn performance_mode() -> Self {
        Self {
            quality: RenderQuality::Low,
            enable_auto_rotation: true,
            rotation_speed: 1.5, // Velocidad para mostrar la escena spa
        }
    }
}