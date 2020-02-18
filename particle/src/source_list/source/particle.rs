pub mod direction;
pub mod position;

#[derive(Copy, Clone)]
pub struct Particle {
    pos:        position::Position,
    prev_pos:   position::Position,
    dir:        direction::Direction,
    sym: char
}

impl Particle {
    pub fn new(pos_x: i32,pos_y: i32,sym: char) -> Particle {
        Particle {
            pos: position::Position::new(pos_x, pos_y),
            prev_pos: position::Position::new(pos_x, pos_y),
            dir: direction::Direction::new(),
            sym: sym
        }
    }
    pub fn par_move(&mut self){
        let (dir_x,dir_y) = self.dir.get_dir_as_cord();
        self.prev_pos.set_pos_x(self.pos.get_pos_x());
        self.prev_pos.set_pos_y(self.pos.get_pos_y());
        self.pos.set_pos_x(self.pos.get_pos_x() + dir_x as i32);
        self.pos.set_pos_y(self.pos.get_pos_y() + dir_y as i32);
    }

    pub fn comp_particle(&self, particle: Particle) -> bool {
        self.pos.comp(*particle.get_position())
    }

    pub fn par_set_pos(&mut self,pos:position::Position){
        self.prev_pos.set_pos_x(self.pos.get_pos_x());
        self.prev_pos.set_pos_y(self.pos.get_pos_y());
        self.pos.set_pos_x(pos.get_pos_x());
        self.pos.set_pos_y(pos.get_pos_y());
    }

    pub fn turn(&mut self,dir: direction::Dir){
        self.dir.turn(dir);
    }
    pub fn change_to(&mut self,dir: direction::Dir){
        self.dir.change_to(dir);
    }

    pub fn get_position(&self) -> &position::Position {
        &self.pos
    }

    pub fn get_prev_position(&self) -> &position::Position {
        &self.prev_pos
    }

    pub fn get_pos_x(&self) -> i32{
        self.pos.get_pos_x()
    }
    pub fn get_pos_y(&self) -> i32{
        self.pos.get_pos_y()
    }
    pub fn get_symbol(&self,as_direction: bool) -> char {
        match as_direction{
            false => self.sym,
            true => self.dir.get_dir_as_symbol()
        }
    }
    pub fn get_dir(&self) -> Result<direction::Dir,&str>{
        self.dir.get_dir()
    }

}