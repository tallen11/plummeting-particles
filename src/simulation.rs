use rand::prelude::ThreadRng;

use crate::particle::{Particle, ParticleType};
use crate::context::{Context, ContextUpdateType};

pub struct Simulation {
    width: usize,
    height: usize,
    particles: Vec<Particle>,
    rng: ThreadRng,
}

impl Simulation {
    pub fn new(size: (usize, usize)) -> Self {
        Self {
            width: size.0,
            height: size.1,
            particles: vec![Particle::new(ParticleType::Empty); size.0*size.1],
            rng: rand::thread_rng(),
        }
    }

    pub fn set_particle(&mut self, particle: Particle, row: usize, col: usize) {
        let index = col + row * self.width;
        self.particles[index] = particle;
    }

    pub fn update(&mut self) {
        for i in 0..self.particles.len() {
            let row = i as u32 / self.width as u32;
            let col = i as u32 % self.width as u32;
           
            if !self.particles[i].is_updated() {
                let mut context = Context::new(&self.particles, &mut self.rng, row, col, self.width as u32, self.height as u32);
                self.particles[i].update(&mut context);

                for update in context.get_updates() {
                    match update {
                        ContextUpdateType::Set(p_type, index) => {
                            self.particles[index as usize].set_type(p_type);
                            self.particles[index as usize].set_updated(true);
                        }
                    }
                }
            } else {
                self.particles[i].set_updated(false);
            }

            // if context.get_updates().len() > 0 {
            // println!("{}", context.get_updates().len());
            // println!("{}, {}", row, col);
            // }
        }
    }

    pub fn render(&self) -> Vec<[u8; 4]> {
        self.particles.iter().map(|particle| {
            match particle.get_type() {
                ParticleType::Empty => [0, 0, 0, 255],
                ParticleType::Wall =>  [255, 255, 255, 255],
                ParticleType::Sand =>  [255, 255, 0, 255],
                ParticleType::Wood =>  [112, 65, 15, 255],
            }
        }).collect()
    }
}
