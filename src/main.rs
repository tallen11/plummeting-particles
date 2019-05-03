extern crate piston_window;
extern crate image;
extern crate rand;

pub mod particle;
pub mod simulation;
pub mod context;

use piston_window::*;
use piston_window::Event::{Loop, Input};
use piston_window::Loop::{Render, Update};
use piston_window::Input::{Button, Move};
use piston_window::Button::{Mouse, Keyboard};
use piston_window::Motion::MouseCursor;

use particle::{Particle, ParticleType};
use simulation::Simulation;

const WINDOW_WIDTH: usize = 300;
const WINDOW_HEIGHT: usize = 300;

const FRAMES_PER_SECOND: u64 = 60;
const UPDATES_PER_SECOND: u64 = 60;

fn main() {
    let mut simulation = Simulation::new((WINDOW_WIDTH, WINDOW_HEIGHT));

    let mut window: PistonWindow = WindowSettings::new("Plummeting Particles", (WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32))
                                                    .exit_on_esc(true)
                                                    .build()
                                                    .unwrap();

    for i in 0..WINDOW_WIDTH {
        simulation.set_particle(Particle::new(ParticleType::Wall), 7 * WINDOW_HEIGHT / 8, i);
    }

    let mut canvas = image::ImageBuffer::new(WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32);
    let mut texture: G2dTexture = Texture::from_image(&mut window.factory, &canvas, &TextureSettings::new()).unwrap();

    let mut event_settings = EventSettings::new();
    event_settings.max_fps = FRAMES_PER_SECOND;
    event_settings.ups = UPDATES_PER_SECOND;

    let mut mouse_down = false;
    let particle_types = vec![ParticleType::Sand,
                              ParticleType::Empty,
                              ParticleType::Wall,
                              ParticleType::Wood,
                              ParticleType::Fire];
    let mut particle_type = ParticleType::Sand;

    let mut events = Events::new(event_settings);
    while let Some(event) = events.next(&mut window) {
        match event {
            Loop(Render(_)) => {
                texture.update(&mut window.encoder, &canvas).unwrap();

                window.draw_2d(&event, |_c, g| {
                    clear([0.0, 0.0, 0.0, 1.0], g);
                                
                    for (index, color) in simulation.render().into_iter().enumerate() {
                        let row = (index / WINDOW_WIDTH) as u32;
                        let col = (index % WINDOW_HEIGHT) as u32;

                        canvas.put_pixel(col, row, image::Rgba(color))
                    }

                    image(&texture, _c.transform, g);
                });
            }

            Loop(Update(update_arts)) => {
                simulation.update();
            }

            Input(Button(button_args)) => {
                match button_args {
                    ButtonArgs { state: ButtonState::Press, button: Mouse(MouseButton::Left), .. } => mouse_down = true,
                    ButtonArgs { state: ButtonState::Release, button: Mouse(MouseButton::Left), .. } => mouse_down = false,
                    
                    ButtonArgs { state: ButtonState::Press, button: Keyboard(key), .. } => {
                        if key.code() >= 48 && key.code() <= 57 {
                            let number = (key.code() - 48) as usize;
                            if number >= particle_types.len() {
                                break;
                            }

                            particle_type = particle_types[number];
                        }
                    },
                    _ => {}
                }
            }

            Input(Move(MouseCursor(x, y))) => {
                if mouse_down {
                    for i in -4..=4 {
                        for j in -4..=4 {
                            let row = y as i32 + i;
                            let col = x as i32 + j;
                            if row < 0 || row >= WINDOW_HEIGHT as i32 || col < 0 || col >= WINDOW_WIDTH as i32 {
                                continue;
                            }

                            simulation.set_particle(Particle::new(particle_type), row as usize, col as usize);
                        }
                    }
                }
            }

            _ => {}
        }
    }                            
}
