use crate::context::Context;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ParticleType {
    Empty,
    Wall,
    Sand,
    Wood,
    Fire,
}

const EMPTY_PARTICLE: Particle = Particle { particle_type: ParticleType::Empty, r_a: 0, updated: false };

impl ParticleType {
    fn update_empty(&self, context: &mut Context) {

    }

    fn update_wall(&self, context: &mut Context) {

    }

    fn update_sand(&self, context: &mut Context) {
        let dir = context.random_dir();
        if context.get(1, 0).get_type() == ParticleType::Empty {
            context.set(EMPTY_PARTICLE, 0, 0);
            context.set(Particle::new(ParticleType::Sand), 1, 0);
        } else if context.get(1, dir).get_type() == ParticleType::Empty {
            context.set(EMPTY_PARTICLE, 0, 0);
            context.set(Particle::new(ParticleType::Sand), 1, dir);
        }
    }

    fn update_wood(&self, context: &mut Context) {

    }

    fn update_fire(&self, context: &mut Context) {
        let p = context.get(0, 0);

        if p.r_a == 0 {
            context.set(EMPTY_PARTICLE, 0, 0);
            return;
        }

        let random_dir_row = context.random_dir();
        let random_dir_col = context.random_dir();
        let n = context.get(random_dir_row, random_dir_col);

        if n.get_type() == ParticleType::Empty {
            // let dur = context.random(0, 2) - 1;

            context.set(EMPTY_PARTICLE, 0, 0);
            context.set(Particle {
                particle_type: ParticleType::Fire,
                r_a: if p.r_a > 50 { 50 } else { p.r_a - 1 },
                updated: false,
            }, random_dir_row, random_dir_col);
        } else if n.get_type() == ParticleType::Wood {
            if context.random_chance(0.1) {
                let extra = context.random(50, 70);

                context.set(Particle {
                    particle_type: ParticleType::Fire,
                    r_a: p.r_a - 1,
                    updated: false,
                }, 0, 0);
                
                context.set(Particle {
                    particle_type: ParticleType::Fire,
                    r_a: p.r_a + extra,
                    updated: false,
                }, random_dir_row, random_dir_col);
            }
        } else {
            // let dur = context.random(0, 2) - 1;

            context.set(Particle {
                particle_type: ParticleType::Fire,
                r_a: p.r_a - 1,
                updated: false,
            }, 0, 0);
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Particle {
    pub particle_type: ParticleType,
    pub r_a: u8,
    pub updated: bool,
}

impl Particle {
    pub fn new(particle_type: ParticleType) -> Self {
        Self {
            particle_type: particle_type,
            r_a: 0,
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
            ParticleType::Fire => self.particle_type.update_fire(context),
        }
    }
}
