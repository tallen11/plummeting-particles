use crate::context::Context;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ParticleType {
    Empty,
    Wall,
    Sand,
    Wood,
}

impl ParticleType {
    fn update_empty(&self, context: &mut Context) {

    }

    fn update_wall(&self, context: &mut Context) {

    }

    fn update_sand(&self, context: &mut Context) {
        let dir = context.random_dir();
        if context.get(1, 0).get_type() == ParticleType::Empty {
            context.set(ParticleType::Empty, 0, 0);
            context.set(ParticleType::Sand, 1, 0);
        } else if context.get(1, dir).get_type() == ParticleType::Empty {
            context.set(ParticleType::Empty, 0, 0);
            context.set(ParticleType::Sand, 1, dir);
        }
    }

    fn update_wood(&self, context: &mut Context) {
        
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Particle {
    particle_type: ParticleType,
    updated: bool,
}

impl Particle {
    pub fn new(particle_type: ParticleType) -> Self {
        Self {
            particle_type: particle_type,
            updated: false,
        }
    }

    pub fn get_type(&self) -> ParticleType {
        self.particle_type
    }

    pub fn set_type(&mut self, particle_type: ParticleType) {
        self.particle_type = particle_type;
    }

    pub fn is_updated(&self) -> bool {
        self.updated
    }

    pub fn set_updated(&mut self, updated: bool) {
        self.updated = updated;
    }

    pub fn update(&self, context: &mut Context) {
        match self.particle_type {
            ParticleType::Empty => self.particle_type.update_empty(context),
            ParticleType::Wall => self.particle_type.update_wall(context),
            ParticleType::Sand => self.particle_type.update_sand(context),
            ParticleType::Wood => self.particle_type.update_wood(context),
        }
    }
}
