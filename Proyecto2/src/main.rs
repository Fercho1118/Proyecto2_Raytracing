// Punto de entrada del programa

use raylib::prelude::*;

mod math;
mod material;
mod geometry;
mod lighting;
mod camera;
mod scene;
mod raytracer;

use math::Vec3;
use material::Material;
use geometry::{Plane, Cube};
use lighting::Light;
use camera::Camera;
use scene::Scene;
use raytracer::Raytracer;

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
    let camera = Camera::new(
        Vec3::new(0.0, 2.0, 5.0),      
        Vec3::new(0.0, 0.0, -3.0),     
        Vec3::up(),                     
        45.0,                           
        SCREEN_WIDTH as f32 / SCREEN_HEIGHT as f32, 
    );

    // Renderizar la escena
    println!("Renderizando escena...");
    let image_buffer = raytracer.render(&scene, &camera);
    println!("Renderizado completo!");


    // Loop principal
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        // Dibujar la imagen renderizada
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                let pixel = image_buffer[y as usize][x as usize];
                d.draw_pixel(x, y, pixel);
            }
        }
        

        // UI
        d.draw_text("Raytracer - Proyecto 2", 10, 10, 20, Color::WHITE);
        d.draw_text(&format!("Resolución: {}x{}", SCREEN_WIDTH, SCREEN_HEIGHT), 10, 35, 20, Color::WHITE);
        d.draw_text("ESC para salir", 10, SCREEN_HEIGHT - 25, 20, Color::WHITE);
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

    // Cubo metálico reflectivo 
    let metal_material = Material::new()
        .with_color(Vec3::new(1.0, 1.0, 1.0))    
        .with_roughness(0.01)                     
        .with_specular(1.0)                       
        .with_reflectivity(0.95);                 
    
    scene.add_cube(Cube::new(
        Vec3::new(0.0, -0.5, -3.0),        
        Vec3::new(1.5, 1.5, 1.5),          
        metal_material,
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

    println!("Escena con cubo metálico reflectivo y 2 luces");
    
    scene
}