// Punto de entrada del programa optimizado

use raylib::prelude::*;

mod math;
mod material;
mod geometry;
mod lighting;
mod camera;
mod scene;
mod raytracer;
mod texture;
mod framebuffer;
mod adaptive_config;

use math::Vec3;
use material::Material;
use geometry::{Cube};
use lighting::Light;
use camera::Camera;
use scene::Scene;
use raytracer::Raytracer;
use texture::Texture;
use framebuffer::Framebuffer;
use adaptive_config::{RenderQuality, AdaptiveConfig};

fn main() {
    println!("Inicializando Raytracer Ultra-Optimizado con Rotación Automática...");

    // Configuración adaptativa (modo performance por defecto)
    let mut config = AdaptiveConfig::performance_mode();
    let mut current_quality = config.quality;
    
    // Configurar Rayon con el número óptimo de threads
    let num_threads = rayon::current_num_threads();
    println!("⚡ Usando {} threads para paralelización", num_threads);
    println!("🎮 Calidad: {}", current_quality.description());

    // Usar dimensiones dinámicas basadas en la calidad
    let (render_width, render_height) = current_quality.dimensions();
    let (display_width, display_height) = (800, 600); // Ventana fija

    // Inicializar ventana con tamaño fijo
    let (mut rl, thread) = raylib::init()
        .size(display_width, display_height)
        .title("Raytracer Ultra-Optimizado - Rotación Auto")
        .build();

    // Crear raytracer con configuración adaptativa
    let mut raytracer = Raytracer::with_config(&config);

    // Crear escena optimizada con cubos más pequeños
    let scene = create_optimized_scene();
    println!("Escena optimizada creada con {} luces", scene.lights.len());
    
    // Crear cámara optimizada para jacuzzi compacto
    let mut camera = Camera::new(
        Vec3::new(-1.0, 1.5, 1.5),    // Más cercana para ver los cubos pequeños
        Vec3::new(0.0, 0.0, 0.0),     // Mirando al centro del jacuzzi
        Vec3::up(),                     
        60.0,                         // FOV amplio para captar toda la escena compacta
        display_width as f32 / display_height as f32, 
    );

    // Variables de control mejoradas
    let mut needs_rerender = true;
    let mut image_buffer = vec![vec![Color::BLACK; display_width as usize]; display_height as usize];
    let mouse_sensitivity = 0.003; // Más suave
    let zoom_speed = 1.5;
    
    // Variables para rotación automática
    let mut auto_time: f32 = 0.0;
    let mut camera_change_timer = 0.0;
    let camera_debounce_time = 0.05;
    let mut manual_control = false; // Si el usuario está controlando manualmente
    let mut last_auto_rotation_time = 0.0; 
    let mut rotation_counter = 0;

    // Framebuffer dinámico
    let mut framebuffer = Framebuffer::new(render_width, render_height);

    // Loop principal ultra-optimizado con rotación automática
    while !rl.window_should_close() {
        let frame_time = rl.get_frame_time();
        auto_time += frame_time;
        
        // Control manual vs automático
        let mut camera_changed = false;
        
        // Controles manuales (desactivan temporalmente la rotación automática)
        if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
                let mouse_delta = rl.get_mouse_delta();
                if mouse_delta.x.abs() > 0.2 || mouse_delta.y.abs() > 0.2 {
                    camera.rotate_around_target(mouse_delta.x, -mouse_delta.y, mouse_sensitivity);
                    camera_changed = true;
                    manual_control = true;
                }
            } else {
                // Solo después de soltar el mouse, permitir rotación automática nuevamente
                if manual_control {
                    manual_control = false;
                    auto_time = 0.0; // Reset para suavizar transición
                }
            }
            
            // Zoom manual (siempre disponible)
            let mut zoom_delta = 0.0;
            if rl.is_key_down(KeyboardKey::KEY_W) || rl.is_key_down(KeyboardKey::KEY_UP) {
                zoom_delta = 1.0;
            }
            if rl.is_key_down(KeyboardKey::KEY_S) || rl.is_key_down(KeyboardKey::KEY_DOWN) {
                zoom_delta = -1.0;
            }
            
            let wheel = rl.get_mouse_wheel_move();
            if wheel.abs() > 0.05 {
                zoom_delta = wheel * 4.0; // Zoom más rápido
            }
            
            if zoom_delta.abs() > 0.05 {
                camera.zoom(zoom_delta, zoom_speed * frame_time);
                camera_changed = true;
            }

            // Rotación automática más conservadora (solo si no hay control manual)
            if config.enable_auto_rotation && !manual_control {
                // Rotar solo cada 500ms para reducir carga de renderizado
                if auto_time - last_auto_rotation_time > 0.5 { 
                    let rotation_amount = config.rotation_speed * 0.5; // Rotación más lenta
                    camera.rotate_around_target(rotation_amount, 0.0, 1.0);
                    camera_changed = true;
                    last_auto_rotation_time = auto_time;
                    rotation_counter += 1;
                    
                    // Debug menos frecuente
                    if rotation_counter % 5 == 0 {
                        println!("Rotación automática: {} pasos", rotation_counter);
                    }
                    
                    // Usar debounce normal para rotación automática
                    camera_change_timer = camera_debounce_time;
                }
            }

            // Cambios de calidad dinámicos
            if rl.is_key_pressed(KeyboardKey::KEY_ONE) {
                current_quality = RenderQuality::Low;
                raytracer.set_quality(current_quality);
                config.quality = current_quality;
                let (new_w, new_h) = current_quality.dimensions();
                framebuffer = Framebuffer::new(new_w, new_h);
                camera_changed = true;
                println!("Calidad: {}", current_quality.description());
            }
            if rl.is_key_pressed(KeyboardKey::KEY_TWO) {
                current_quality = RenderQuality::Medium;
                raytracer.set_quality(current_quality);
                config.quality = current_quality;
                let (new_w, new_h) = current_quality.dimensions();
                framebuffer = Framebuffer::new(new_w, new_h);
                camera_changed = true;
                println!("Calidad: {}", current_quality.description());
            }
            if rl.is_key_pressed(KeyboardKey::KEY_THREE) {
                current_quality = RenderQuality::High;
                raytracer.set_quality(current_quality);
                config.quality = current_quality;
                let (new_w, new_h) = current_quality.dimensions();
                framebuffer = Framebuffer::new(new_w, new_h);
                camera_changed = true;
                println!("Calidad: {}", current_quality.description());
            }
            
            // Toggle rotación automática
            if rl.is_key_pressed(KeyboardKey::KEY_R) {
                config.enable_auto_rotation = !config.enable_auto_rotation;
                println!("Rotación automática: {}", if config.enable_auto_rotation { "ON" } else { "OFF" });
            }
        
        // Sistema de debounce optimizado (más agresivo para rotación automática)
        if camera_changed {
            if manual_control {
                camera_change_timer = camera_debounce_time; // Debounce normal para control manual
            } else {
                camera_change_timer = 0.0; // Sin debounce para rotación automática
            }
        } else if camera_change_timer > 0.0 {
            camera_change_timer -= frame_time;
        }
        
        // Renderizado ultra-optimizado con escalado dinámico
        if needs_rerender || (camera_change_timer <= 0.0 && camera_change_timer > -0.1) {
            // Menos verbose para rotación automática
            if manual_control || needs_rerender {
                println!("\nIniciando renderizado paralelo {} ({}x{})...", 
                    current_quality.description(), raytracer.width, raytracer.height);
            }
            
            // Renderizado directo al framebuffer apropiado
            raytracer.render_to_framebuffer(&scene, &camera, &mut framebuffer);
            
            // Convertir y escalar al tamaño de ventana
            image_buffer = scale_framebuffer_to_window(&framebuffer, display_width as u32, display_height as u32);
            
            needs_rerender = false;
            camera_change_timer = -1.0;
        }

        // Dibujo optimizado con información mejorada
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::new(15, 15, 25, 255)); // Fondo más elegante

        // Dibujar imagen escalada
        for y in 0..display_height {
            for x in 0..display_width {
                let pixel = image_buffer[y as usize][x as usize];
                d.draw_pixel(x, y, pixel);
            }
        }
        
        // UI Ultra-mejorada con información completa
        if camera_change_timer > 0.0 {
            draw_waiting_ui(&mut d, camera_change_timer, camera_debounce_time);
        } else if camera_change_timer > -0.1 {
            draw_ready_ui(&mut d);
        } else {
            draw_controls_ui(&mut d, &camera, &num_threads, &current_quality, &config, manual_control);
        }
    }
}

// Funciones auxiliares para UI simplificada
fn draw_waiting_ui(d: &mut RaylibDrawHandle, _timer: f32, _total_time: f32) {
    // Simplificado - sin barra de progreso
    d.draw_rectangle(0, 0, 150, 40, Color::new(0, 0, 0, 150));
    d.draw_text("Preparando...", 10, 10, 14, Color::ORANGE);
}

fn draw_ready_ui(d: &mut RaylibDrawHandle) {
    // Mantener el mensaje de listo pero más pequeño
    d.draw_rectangle(0, 0, 120, 30, Color::new(0, 0, 0, 100));
    d.draw_text("Listo!", 10, 8, 14, Color::LIME);
}

fn draw_controls_ui(d: &mut RaylibDrawHandle, _camera: &Camera, _num_threads: &usize, _quality: &RenderQuality, config: &AdaptiveConfig, _manual: bool) {
    let screen_height = d.get_screen_height();
    
    // UI súper simplificada - solo lo esencial en una línea
    d.draw_rectangle(0, screen_height - 50, d.get_screen_width(), 50, Color::new(0, 0, 0, 120));
    
    // Solo controles básicos
    d.draw_text("W/S para zoom | Scroll para zoom", 10, screen_height - 40, 14, Color::WHITE);
    
    // Estado de rotación automática (lo más importante)
    let rotation_status = if config.enable_auto_rotation { 
        "ROTACIÓN AUTOMÁTICA ACTIVA" 
    } else { 
        "Rotación OFF" 
    };
    let rotation_color = if config.enable_auto_rotation { Color::LIME } else { Color::DARKGRAY };
    
    d.draw_text(rotation_status, 10, screen_height - 22, 16, rotation_color);
}

// Función para escalar framebuffer al tamaño de ventana
fn scale_framebuffer_to_window(framebuffer: &Framebuffer, target_width: u32, target_height: u32) -> Vec<Vec<raylib::prelude::Color>> {
    let fb_buffer = framebuffer.to_raylib_buffer();
    let fb_height = fb_buffer.len();
    let fb_width = if fb_height > 0 { fb_buffer[0].len() } else { 0 };
    
    let mut scaled_buffer = Vec::with_capacity(target_height as usize);
    
    for y in 0..target_height {
        let mut row = Vec::with_capacity(target_width as usize);
        for x in 0..target_width {
            // Mapeo bilinear simple
            let fb_x = ((x as f32 / target_width as f32) * fb_width as f32) as usize;
            let fb_y = ((y as f32 / target_height as f32) * fb_height as f32) as usize;
            
            let fb_x = fb_x.min(fb_width.saturating_sub(1));
            let fb_y = fb_y.min(fb_height.saturating_sub(1));
            
            let pixel = fb_buffer[fb_y][fb_x];
            row.push(pixel);
        }
        scaled_buffer.push(row);
    }
    
    scaled_buffer
}

// Crea una escena ultra-optimizada estilo jacuzzi spa
fn create_optimized_scene() -> Scene {
    let mut scene = Scene::new();
    
    // Color de fondo claro para resaltar el agua azul
    scene.set_background_color(Vec3::new(0.8, 0.9, 0.95)); // Celeste muy claro

    // === MATERIALES PARA JACUZZI REAL CON AGUA AZUL ===
    
    // AGUA AZUL VERDADERA (Color jacuzzi real)
    let agua_material = Material::new()
        .with_color(Vec3::new(0.1, 0.4, 0.8)) // AZUL INTENSO COMO AGUA REAL
        .with_specular(0.8)
        .with_roughness(0.1)
        .with_reflectivity(0.3)      
        .with_transparency(0.6)      // Semi-transparente para ver profundidad
        .with_refractive_index(1.33); // Índice del agua real
    
    // MADERA DE SPA (Deck del jacuzzi)
    let madera_texture = match Texture::from_file("assets/img/wood.jpg") {
        Ok(texture) => texture,
        Err(_) => Texture::solid_color(Vec3::new(0.65, 0.4, 0.25)) // Madera cálida
    };
    let madera_material = Material::new()
        .with_texture(madera_texture)
        .with_specular(0.1)
        .with_roughness(0.7)
        .with_reflectivity(0.05);

    // MÁRMOL ELEGANTE (Piso del spa)
    let marmol_material = Material::new()
        .with_color(Vec3::new(0.9, 0.9, 0.85)) // Mármol blanco cálido
        .with_specular(0.6)
        .with_roughness(0.15)
        .with_reflectivity(0.4); // Bien reflectivo como mármol real

    // LADRILLO RÚSTICO (Paredes decorativas)
    let ladrillo_texture = match Texture::from_file("assets/img/brick.jpg") {
        Ok(texture) => texture,
        Err(_) => Texture::solid_color(Vec3::new(0.7, 0.35, 0.2))
    };
    let ladrillo_material = Material::new()
        .with_texture(ladrillo_texture)
        .with_roughness(0.8)
        .with_specular(0.15)
        .with_reflectivity(0.03);

    // PIEDRA NATURAL (Elementos decorativos)
    let piedra_texture = match Texture::from_file("assets/img/cobblestone.png") {
        Ok(texture) => texture,
        Err(_) => Texture::solid_color(Vec3::new(0.4, 0.4, 0.45))
    };
    let piedra_material = Material::new()
        .with_texture(piedra_texture)
        .with_specular(0.05)
        .with_roughness(0.9)
        .with_reflectivity(0.02);

    // METAL BRILLANTE (Elementos lujosos)
    let metal_material = Material::new()
        .with_color(Vec3::new(0.85, 0.85, 0.9)) // Acero inoxidable
        .with_specular(0.9)
        .with_roughness(0.05)
        .with_reflectivity(0.75);

    // === CONSTRUCCIÓN DE JACUZZI COMPACTO REAL ===
    
    // Cubos SÚPER PEQUEÑOS para ultra-performance y realismo
    let mini_cube = Vec3::new(0.3, 0.3, 0.3);     // Extra pequeños
    let small_cube = Vec3::new(0.4, 0.4, 0.4);    // Pequeños
    
    // JACUZZI CENTRAL - 4 cubos de AGUA AZUL PEGADOS (2x2)
    // ¡COMPLETAMENTE PEGADOS SIN ESPACIO!
    scene.add_cube(Cube::new(
        Vec3::new(-0.15, 0.1, -0.15), // PEGADOS - separación de solo 0.3 
        mini_cube,
        agua_material.clone(),
    ));
    scene.add_cube(Cube::new(
        Vec3::new(0.15, 0.1, -0.15),  // PEGADOS completamente
        mini_cube,
        agua_material.clone(),
    ));
    scene.add_cube(Cube::new(
        Vec3::new(-0.15, 0.1, 0.15),  // PEGADOS completamente
        mini_cube,
        agua_material.clone(),
    ));
    scene.add_cube(Cube::new(
        Vec3::new(0.15, 0.1, 0.15),   // PEGADOS completamente
        mini_cube,
        agua_material,
    ));

    // DECK DE MADERA COMPACTO (Marco perfecto alrededor)
    let deck_y = -0.1; // Justo debajo del agua
    
    // Marco compacto de 3x3 con hueco en el centro (para el agua)
    // Frente
    scene.add_cube(Cube::new(Vec3::new(-0.5, deck_y, -0.5), small_cube, madera_material.clone()));
    scene.add_cube(Cube::new(Vec3::new(0.0, deck_y, -0.5), small_cube, madera_material.clone()));
    scene.add_cube(Cube::new(Vec3::new(0.5, deck_y, -0.5), small_cube, madera_material.clone()));
    
    // Lados (sin centro)
    scene.add_cube(Cube::new(Vec3::new(-0.5, deck_y, 0.0), small_cube, madera_material.clone()));
    scene.add_cube(Cube::new(Vec3::new(0.5, deck_y, 0.0), small_cube, madera_material.clone()));
    
    // Atrás
    scene.add_cube(Cube::new(Vec3::new(-0.5, deck_y, 0.5), small_cube, madera_material.clone()));
    scene.add_cube(Cube::new(Vec3::new(0.0, deck_y, 0.5), small_cube, madera_material.clone()));
    scene.add_cube(Cube::new(Vec3::new(0.5, deck_y, 0.5), small_cube, madera_material));

    // PISO DE MÁRMOL COMPACTO (5x5 grid alrededor del deck)
    let piso_y = -0.3;
    // Cuadrado 5x5 con el jacuzzi en el centro
    for i in -2i32..=2i32 {
        for j in -2i32..=2i32 {
            // Saltar el área del deck (centro 3x3)
            if i.abs() <= 1 && j.abs() <= 1 {
                continue; // El deck ya ocupa esta área
            }
            scene.add_cube(Cube::new(
                Vec3::new(i as f32 * 0.4, piso_y, j as f32 * 0.4), 
                small_cube, 
                marmol_material.clone()
            ));
        }
    }

    // ELEMENTOS DECORATIVOS MINIMALISTAS
    // Torres pequeñas de ladrillo (esquinas exteriores)
    scene.add_cube(Cube::new(Vec3::new(-1.2, 0.1, -1.2), mini_cube, ladrillo_material.clone()));
    scene.add_cube(Cube::new(Vec3::new(1.2, 0.1, -1.2), mini_cube, ladrillo_material.clone()));
    scene.add_cube(Cube::new(Vec3::new(-1.2, 0.1, 1.2), mini_cube, ladrillo_material.clone()));
    scene.add_cube(Cube::new(Vec3::new(1.2, 0.1, 1.2), mini_cube, ladrillo_material));
    
    // Rocas decorativas pequeñas
    scene.add_cube(Cube::new(Vec3::new(-1.0, -0.2, 0.0), mini_cube, piedra_material.clone()));
    scene.add_cube(Cube::new(Vec3::new(1.0, -0.2, 0.0), mini_cube, piedra_material));
    
    // Accesorio metálico pequeño (como grifo o lámpara)
    scene.add_cube(Cube::new(Vec3::new(0.0, 0.3, -0.8), mini_cube, metal_material));

    // === ILUMINACIÓN TIPO SPA RELAJANTE ===
    
    // Luz principal cálida (simulando atardecer)
    scene.add_light(Light::new(
        Vec3::new(-4.0, 6.0, -2.0),     
        Vec3::new(1.0, 0.9, 0.8),       // Luz cálida dorada
        2.2,                            
    ));

    // Luz secundaria azulada (para resaltar el agua)
    scene.add_light(Light::new(
        Vec3::new(4.0, 4.0, 2.0),     
        Vec3::new(0.8, 0.9, 1.0),       // Luz azul suave
        1.8,                           
    ));

    // Luz ambiental suave desde arriba
    scene.add_light(Light::new(
        Vec3::new(0.0, 8.0, 0.0),      
        Vec3::new(0.9, 0.9, 0.95),      // Luz neutra
        1.2,                           
    ));

    println!("Escena SPA JACUZZI creada:");
    println!("   Jacuzzi 2x2 con agua cristalina (refracción)");
    println!("   Deck de madera natural");
    println!("   Piso de mármol reflectivo");
    println!("   Elementos decorativos (ladrillo + piedra + metal)");
    println!("   Iluminación tipo spa (3 luces ambientales)");
    println!("Optimizada para máximo rendimiento visual");
    
    scene
}