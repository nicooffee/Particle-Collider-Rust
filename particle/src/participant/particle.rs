pub mod direction;
pub mod position;


pub struct Particle {
    pos: position::Position,
    dir: direction::Direction,
    sym: char
}

impl Particle {
    pub fn new(pos_x: i32,pos_y: i32,sym: char) -> Particle {
        Particle {
            pos: position::Position::new(pos_x, pos_y),
            dir: direction::Direction::new(),
            sym: sym
        }
    }
    pub fn par_move(&mut self){
        let (dir_x,dir_y) = self.dir.get_dir_as_cord();
        self.pos.set_pos_x(self.pos.get_pos_x() + dir_x as i32);
        self.pos.set_pos_y(self.pos.get_pos_y() + dir_y as i32);
    }

    pub fn par_set_pos(&mut self,pos:position::Position){
        self.pos.set_pos_x(pos.get_pos_x());
        self.pos.set_pos_y(pos.get_pos_y());
    }

    pub fn get_pos_x(&self) -> i32{
        self.pos.get_pos_x()
    }
    pub fn get_pos_y(&self) -> i32{
        self.pos.get_pos_y()
    }
    pub fn get_sym(&self,as_direction: bool) -> char {
        match as_direction{
            false => self.sym,
            true => self.dir.get_dir_as_symbol()
        }
    }
    pub fn get_dir(&self) -> Result<u16,&str>{
        self.dir.get_dir()
    }

    pub fn move_45(&mut self){
        self.dir.move_45()
    }
    pub fn move_90(&mut self){
        self.dir.move_90()
    }
    pub fn move_135(&mut self){
        self.dir.move_135()
    }
    pub fn move_180(&mut self){
        self.dir.move_180()
    }
    pub fn move_225(&mut self){
        self.dir.move_225()
    }
    pub fn move_270(&mut self){
        self.dir.move_270()
    }
    pub fn move_315(&mut self){
        self.dir.move_315()
    }
}