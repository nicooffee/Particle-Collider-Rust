use termion_ext::AdvWrite;
use termion::screen::AlternateScreen;

pub mod direction;
pub mod position;

#[derive(Copy, Clone)]
pub struct GenericParticle {
    pos:        position::Position,
    prev_pos:   position::Position,
    dir:        direction::Direction,
    sym: char
}

pub trait Particle {
    fn par_move(&mut self);
    fn par_print<W: std::io::Write>(&self,srn: &mut AlternateScreen<W>,as_dir: bool);
    fn comp_particle<T: Particle>(&self, particle: T) -> bool;
    fn par_set_pos(&mut self,pos:position::Position);
    fn turn(&mut self,dir: direction::Dir);
    fn change_to(&mut self,dir: direction::Dir);
    fn get_position(&self) -> &position::Position;
    fn get_prev_position(&self) -> &position::Position;
    fn get_pos_x(&self) -> i32;
    fn get_pos_y(&self) -> i32;
    fn get_symbol(&self,as_direction: bool) -> char;
    fn get_dir(&self) -> Result<direction::Dir,&str>;
}





impl GenericParticle {
    pub fn new(pos_x: i32,pos_y: i32,sym: char) -> GenericParticle {
        GenericParticle {
            pos: position::Position::new(pos_x, pos_y),
            prev_pos: position::Position::new(pos_x, pos_y),
            dir: direction::Direction::new(),
            sym: sym
        }
    }
}

impl Particle for GenericParticle {
    fn par_move(&mut self){
        let (dir_x,dir_y) = self.dir.get_dir_as_cord();
        self.prev_pos.set_pos_x(self.pos.get_pos_x());
        self.prev_pos.set_pos_y(self.pos.get_pos_y());
        self.pos.set_pos_x(self.pos.get_pos_x() + dir_x as i32);
        self.pos.set_pos_y(self.pos.get_pos_y() + dir_y as i32);
    }
    fn par_print<W: std::io::Write>(&self,srn: &mut AlternateScreen<W>,as_dir: bool){
        let p_prev_src = self.get_prev_position();
        let p_src = self.get_position();
        srn.w_go_str(p_prev_src.get_pos_x()as u16,p_prev_src.get_pos_y()as u16,String::from(" "));
        srn.w_go_str(p_src.get_pos_x()as u16,p_src.get_pos_y()as u16,self.get_symbol(as_dir).to_string());
    }
    fn comp_particle<T: Particle>(&self, particle: T) -> bool {
        self.pos.comp(*particle.get_position())
    }
    
    fn par_set_pos(&mut self,pos:position::Position){
        self.prev_pos.set_pos_x(self.pos.get_pos_x());
        self.prev_pos.set_pos_y(self.pos.get_pos_y());
        self.pos.set_pos_x(pos.get_pos_x());
        self.pos.set_pos_y(pos.get_pos_y());
    }
    
    fn turn(&mut self,dir: direction::Dir){
        self.dir.turn(dir);
    }
    
    fn change_to(&mut self,dir: direction::Dir){
        self.dir.change_to(dir);
    }
    
    fn get_position(&self) -> &position::Position {
        &self.pos
    }
    
    fn get_prev_position(&self) -> &position::Position {
        &self.prev_pos
    }
    
    fn get_pos_x(&self) -> i32{
        self.pos.get_pos_x()
    }
    
    fn get_pos_y(&self) -> i32{
        self.pos.get_pos_y()
    }
    
    fn get_symbol(&self,as_direction: bool) -> char {
        match as_direction{
            false => self.sym,
            true => self.dir.get_dir_as_symbol()
        }
    }
    
    fn get_dir(&self) -> Result<direction::Dir,&str>{
        self.dir.get_dir()
    }
}