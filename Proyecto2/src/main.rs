// Punto de entrada del programa

use raylib::prelude::*;

mod math;
mod material;
mod geometry;
mod lighting;
mod camera;
mod scene;
mod raytracer;
mod texture;

use math::Vec3;
use material::Material;
use geometry::{Plane, Cube};
use lighting::Light;
use camera::Camera;
use scene::Scene;
use raytracer::Raytracer;
use texture::Texture;

// Constantes de la ventana
const SCREEN_WIDTH: i32 = 800;
const SCREEN_HEIGHT: i32 = 600;

fn main() {
    println!("Inicializando Raytracer...");

    // Inicializar ventana
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Raytracer - Proyecto 2")
        .build();

    // Crear raytracer
    let raytracer = Raytracer::new(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32);

    // Crear escena de prueba
    let scene = create_test_scene();
    println!("Escena creada con {} luces", scene.lights.len());
    
    // Crear cámara
    let mut camera = Camera::new(
        Vec3::new(0.0, 2.0, 5.0),      
        Vec3::new(0.0, 0.0, -3.0),     
        Vec3::up(),                     
        45.0,                           
        SCREEN_WIDTH as f32 / SCREEN_HEIGHT as f32, 
    );

    // Variables para controles
    let mut needs_rerender = true;
    let mut image_buffer = vec![vec![Color::BLACK; SCREEN_WIDTH as usize]; SCREEN_HEIGHT as usize];
    let mouse_sensitivity = 0.005;
    let zoom_speed = 2.0;
    
    // Variables para debounce (evitar renderizado constante)
    let mut camera_change_timer = 0.0;
    let camera_debounce_time = 0.3; // Esperar 0.3 segundos después del último cambio
    let mut is_rendering = false;

    // Loop principal
    while !rl.window_should_close() {
        let frame_time = rl.get_frame_time();
        
        // Controles de cámara
        let mut camera_changed = false;
        
        // Solo permitir controles si no está renderizando
        if !is_rendering {
            // Rotación con mouse (cuando se mantiene presionado el botón izquierdo)
            if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
                let mouse_delta = rl.get_mouse_delta();
                if mouse_delta.x.abs() > 0.5 || mouse_delta.y.abs() > 0.5 {
                    camera.rotate_around_target(mouse_delta.x, -mouse_delta.y, mouse_sensitivity);
                    camera_changed = true;
                }
            }
            
            // Zoom con W/S o flechas arriba/abajo
            let mut zoom_delta = 0.0;
            if rl.is_key_down(KeyboardKey::KEY_W) || rl.is_key_down(KeyboardKey::KEY_UP) {
                zoom_delta = 1.0;
            }
            if rl.is_key_down(KeyboardKey::KEY_S) || rl.is_key_down(KeyboardKey::KEY_DOWN) {
                zoom_delta = -1.0;
            }
            
            // Zoom con rueda del mouse
            let wheel = rl.get_mouse_wheel_move();
            if wheel.abs() > 0.1 {
                zoom_delta = wheel * 3.0;
            }
            
            if zoom_delta.abs() > 0.1 {
                camera.zoom(zoom_delta, zoom_speed * frame_time);
                camera_changed = true;
            }
        }
        
        // Sistema de debounce simplificado
        if camera_changed {
            camera_change_timer = camera_debounce_time; // Reiniciar timer
        } else if camera_change_timer > 0.0 {
            camera_change_timer -= frame_time; // Decrementar timer
        }
        
        // Re-renderizar cuando termine el timer o es el primer frame
        if needs_rerender || (camera_change_timer <= 0.0 && camera_change_timer > -0.1) {
            is_rendering = true;
            println!("\nRenderizando...");
            image_buffer = raytracer.render(&scene, &camera);
            is_rendering = false;
            needs_rerender = false;
            camera_change_timer = -1.0; // Marcar como completado
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        // Dibujar la imagen renderizada
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                let pixel = image_buffer[y as usize][x as usize];
                d.draw_pixel(x, y, pixel);
            }
        }
        
        // UI simplificada - solo mostrar si está renderizando o controles básicos
        if is_rendering {
            // Mostrar estado de renderizado
            d.draw_rectangle(0, 0, SCREEN_WIDTH, 80, Color::new(0, 0, 0, 200));
            d.draw_text("Renderizando... Espera por favor", 10, 10, 24, Color::WHITE);
            d.draw_text("(No muevas la cámara hasta que termine)", 10, 40, 18, Color::LIGHTGRAY);
        } else {
            // Controles activos
            if camera_change_timer > 0.0 {
                d.draw_rectangle(0, 0, 400, 80, Color::new(0, 0, 0, 150));
                d.draw_text(&format!("⏱Esperando: {:.1}s", camera_change_timer), 10, 10, 18, Color::YELLOW);
                d.draw_text("Puedes seguir moviendo la cámara...", 10, 35, 16, Color::LIGHTGRAY);
                let progress = ((camera_debounce_time - camera_change_timer) / camera_debounce_time * 20.0) as i32;
                let bar = "█".repeat(progress as usize) + &"░".repeat((20 - progress) as usize);
                d.draw_text(&format!("[{}]", bar), 10, 55, 14, Color::GREEN);
            } else if camera_change_timer > -0.1 {
                d.draw_rectangle(0, 0, 200, 40, Color::new(0, 0, 0, 150));
                d.draw_text("Listo para renderizar", 10, 10, 16, Color::GREEN);
            } else {
                // Controles normales
                d.draw_rectangle(0, SCREEN_HEIGHT - 50, SCREEN_WIDTH, 50, Color::new(0, 0, 0, 100));
                d.draw_text("Arrastra para rotar | W/S para zoom | ESC para salir", 10, SCREEN_HEIGHT - 35, 16, Color::WHITE);
                d.draw_text(&format!(" Pos: ({:.1}, {:.1}, {:.1})", camera.position.x, camera.position.y, camera.position.z), 
                          10, SCREEN_HEIGHT - 15, 14, Color::LIGHTGRAY);
            }
        }
    }
}

// Crea una escena simple con un solo cubo metálico
fn create_test_scene() -> Scene {
    let mut scene = Scene::new();
    
    // Configurar color de fondo simple
    scene.set_background_color(Vec3::new(0.2, 0.3, 0.8));
    
    // Agregar plano 
    let floor_material = Material::new()
        .with_color(Vec3::new(0.3, 0.7, 0.2))  
        .with_roughness(0.9);
    
    scene.add_plane(Plane::new(
        Vec3::new(0.0, -2.0, 0.0),    
        Vec3::up(),                    
        floor_material,
    ));

    // Cubo con textura de ladrillo real desde imagen
    let brick_texture = match Texture::from_file("assets/img/brick.jpg") {
        Ok(texture) => texture,
        Err(e) => {
            println!("Error cargando textura: {}. Usando color sólido de respaldo.", e);
            Texture::solid_color(Vec3::new(0.8, 0.4, 0.2)) // Color de respaldo
        }
    };
    let brick_material = Material::new()
        .with_texture(brick_texture)
        .with_roughness(0.7)                     
        .with_specular(0.2)                       
        .with_reflectivity(0.05);                
    
    scene.add_cube(Cube::new(
        Vec3::new(0.0, -0.5, -3.0),        
        Vec3::new(1.5, 1.5, 1.5),          
        brick_material,
    ));

    // Luz principal intensa y posicionada para reflejos metálicos
    scene.add_light(Light::new(
        Vec3::new(-3.0, 5.0, 2.0),     
        Vec3::new(1.0, 1.0, 0.9),      
        1.5,                          
    ));

    // Segunda luz para crear mejor reflejo
    scene.add_light(Light::new(
        Vec3::new(3.0, 4.0, -2.0),     
        Vec3::new(0.8, 0.9, 1.0),      
        0.8,                           
    ));

    println!("Escena con cubo de ladrillo REAL (foto JPG) y 2 luces");
    
    scene
}