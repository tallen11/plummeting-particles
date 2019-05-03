use rand::prelude::ThreadRng;
use rand::Rng;

use crate::particle::{Particle, ParticleType};

#[derive(Copy, Clone, Debug)]
pub enum ContextUpdateType {
    Set(Particle, u32),
}

pub struct Context<'a> {
    updates: Vec<ContextUpdateType>,
    simulation_particles: &'a Vec<Particle>,
    rng: &'a mut ThreadRng,
    row: u32,
    col: u32,
    simulation_width: u32,
    simulation_height: u32,
}

impl<'a> Context<'a> {
    pub fn new(simulation_particles: &'a Vec<Particle>, rng: &'a mut ThreadRng, row: u32, col: u32, simulation_width: u32, simulation_height: u32) -> Self {
        Self {
            updates: Vec::new(),
            simulation_particles: simulation_particles,
            rng: rng,
            row: row,
            col: col,
            simulation_width: simulation_width,
            simulation_height: simulation_height,
        }
    }

    pub fn set(&mut self, particle: Particle, dr: i32, dc: i32) {
        let row = self.row as i32 + dr;
        let col = self.col as i32 + dc;

        if row < 0 || row >= self.simulation_height as i32 || col <= 0 || col >= self.simulation_width as i32 {
            return;
        }

        let index = col + row * self.simulation_width as i32;
        self.updates.push(ContextUpdateType::Set(particle, index as u32));
    }

    pub fn get(&self, dr: i32, dc: i32) -> Particle {
        let row = self.row as i32 + dr;
        let col = self.col as i32 + dc;

        if row < 0 || row >= self.simulation_height as i32 || col <= 0 || col >= self.simulation_width as i32 {
            return Particle::new(ParticleType::Empty);
        }

        let index = col + row * self.simulation_width as i32;
        self.simulation_particles[index as usize]
    }

    pub fn random_dir(&mut self) -> i32 {
        self.rng.gen_range(0, 3) - 1
    }

    pub fn get_updates(&self) -> Vec<ContextUpdateType> {
        self.updates.to_vec()
    }
}
