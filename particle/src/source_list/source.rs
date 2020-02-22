pub mod particle;
pub mod limit_box;
use limit_box::Area;
use particle::Particle;
use particle::direction::Dir;
use rand::distributions::{Distribution, Uniform};
use termion::screen::AlternateScreen;

pub struct Source {
    id: String,
    particle: particle::GenericParticle,
    c_particle: u32,
    limits: limit_box::LimitBox
}

impl Source {
    pub fn new(id: String,pos_x:Option<i32>,pos_y:Option<i32>,sym:Option<char>,
        c_particle:u32,
        l_min_x:i32,
        l_min_y:i32,
        l_max_x:i32,
        l_max_y:i32
        ) -> Source{
            Source{
                id: id,
                particle: particle::GenericParticle::new(
                    if let Some(x) = pos_x {x} else {l_min_x},
                    if let Some(x) = pos_y {x} else {l_min_y},
                    if let Some(c) = sym {c} else {'*'}
                ),
                c_particle: c_particle,
                limits: limit_box::LimitBox::new(l_min_x, l_min_y, l_max_x, l_max_y)
            }
    }
    
    pub fn particle_move(&mut self) {
        let mut par_aux = self.particle;
        par_aux.par_move();
        let mut rng = rand::thread_rng();
        let die = Uniform::from(0..3);
        match self.limits.area_point(par_aux.get_pos_x(),par_aux.get_pos_y()).unwrap(){
            Area::Inside => (),
            Area::OutSide1 => self.particle.change_to(
                match die.sample(&mut rng){0=> Dir::D135, 1=> Dir::D180,  2=>Dir::D225,_ => panic!("awas")}
            ),
            Area::OutSide2 => self.particle.change_to(
                match die.sample(&mut rng){0=> Dir::D225, 1=> Dir::D270,  2=>Dir::D315,_ => panic!("awas")}
            ), 
            Area::OutSide3 => self.particle.change_to(
                match die.sample(&mut rng){0=> Dir::D45, 1=> Dir::D0,  2=>Dir::D315,_ => panic!("awas")}
            ),
            Area::OutSide4 => self.particle.change_to(
                match die.sample(&mut rng){0=> Dir::D135, 1=> Dir::D90,  2=>Dir::D45,_ => panic!("awas")}
            ),
            Area::OutCorner1 => self.particle.change_to(Dir::D225),
            Area::OutCorner2 => self.particle.change_to(Dir::D315),
            Area::OutCorner3 => self.particle.change_to(Dir::D45),
            Area::OutCorner4 => self.particle.change_to(Dir::D135)
        }
        self.particle.par_move();
    }

    pub fn particle_clear<W: std::io::Write>(&self,srn: &mut AlternateScreen<W>){
        self.particle.par_clear(srn);
    }

    pub fn particle_print<W: std::io::Write>(&self,srn: &mut AlternateScreen<W>,as_dir: bool){
        self.particle.par_print(srn,as_dir);
    }

    pub fn sub_particle(&mut self) -> u32 {
        if self.c_particle>0 {
            self.c_particle = self.c_particle -1;
        }
        self.c_particle
    }

    pub fn set_rand_pos(&mut self) {
        let (pos_x,pos_y) = self.limits.get_rand_cord();
        let n_dir: Dir = Dir::rand();
        self.particle.par_set_pos(particle::position::Position::new(pos_x,pos_y));
        self.particle.change_to(n_dir);
    }

    pub fn comp_particle(&self, source: &Source) -> bool {
        self.particle.comp_particle(source.particle)
    }

    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    pub fn get_symbol(&self,as_direction: bool) -> char {
        self.particle.get_symbol(as_direction)
    }

    pub fn get_position(&self) -> &particle::position::Position {
        self.particle.get_position()
    }
    pub fn get_prev_position(&self) -> &particle::position::Position {
        self.particle.get_prev_position()
    }
    pub fn get_c_particle(&self) -> u32 {
        self.c_particle
    }
}