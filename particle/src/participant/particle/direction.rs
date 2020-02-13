#[derive(Copy, Clone)]
pub struct Direction {
    dir_x: i8,
    dir_y: i8
}

impl Direction {
    pub fn new() -> Direction{
        Direction{
            dir_x: 1,
            dir_y: 0
        }
    }
    pub fn get_dir(&self) -> Result<u16,&str>{
        match self{
            Direction{dir_x:1,dir_y:0}      => Ok(0),
            Direction{dir_x:1,dir_y:-1}     => Ok(45),
            Direction{dir_x:0,dir_y:-1}     => Ok(90),
            Direction{dir_x:-1,dir_y:-1}    => Ok(135),
            Direction{dir_x:-1,dir_y:0}     => Ok(180),
            Direction{dir_x:-1,dir_y:1}     => Ok(225),
            Direction{dir_x:0,dir_y:1}      => Ok(270),
            Direction{dir_x:1,dir_y:1}      => Ok(315),
            _ => Err("Direccion no definida")
        }
    }
    pub fn change_to_0(&mut self){
        self.dir_x = 1; self.dir_y = 0
    }
    pub fn change_to_45(&mut self) {
        self.dir_x = 1; self.dir_y = -1;
    }
    pub fn change_to_90(&mut self) {
        self.dir_x = 0; self.dir_y = -1;
    }
    pub fn change_to_135(&mut self) {
        self.dir_x = -1; self.dir_y = -1;
    }
    pub fn change_to_180(&mut self) {
        self.dir_x = -1; self.dir_y = 0;
    }
    pub fn change_to_225(&mut self) {
        self.dir_x = -1; self.dir_y = 1;
    }
    pub fn change_to_270(&mut self) {
        self.dir_x = 0; self.dir_y = 1;
    }
    pub fn change_to_315(&mut self) {
        self.dir_x = 1; self.dir_y = 1;
    }
    
    pub fn get_dir_as_cord(&self) -> (i8,i8){
        (self.dir_x,self.dir_y)
    }

    pub fn get_dir_as_symbol(&self) -> char{
        match self{       
            Direction{dir_x:1,dir_y:0}      => '→',
            Direction{dir_x:1,dir_y:-1}     => '↗',
            Direction{dir_x:0,dir_y:-1}     => '↑',
            Direction{dir_x:-1,dir_y:-1}    => '↖',
            Direction{dir_x:-1,dir_y:0}     => '←',
            Direction{dir_x:-1,dir_y:1}     => '↙',
            Direction{dir_x:0,dir_y:1}      => '↓',
            Direction{dir_x:1,dir_y:1}      => '↘',
            _ => '?'
        }
    }

    pub fn turn_45(&mut self){
        match self{
            Direction{dir_x:1,dir_y:0}      => {self.dir_x=1;self.dir_y=-1},
            Direction{dir_x:1,dir_y:-1}     => {self.dir_x=0;self.dir_y=-1},
            Direction{dir_x:0,dir_y:-1}     => {self.dir_x=-1;self.dir_y=-1},
            Direction{dir_x:-1,dir_y:-1}    => {self.dir_x=-1;self.dir_y=0},
            Direction{dir_x:-1,dir_y:0}     => {self.dir_x=-1;self.dir_y=1},
            Direction{dir_x:-1,dir_y:1}     => {self.dir_x=0;self.dir_y=1},
            Direction{dir_x:0,dir_y:1}      => {self.dir_x=1;self.dir_y=1},
            Direction{dir_x:1,dir_y:1}      => {self.dir_x=1;self.dir_y=0},
            _ => ()
        }
    }
    pub fn turn_90(&mut self){
        for _ in 1..=2{ self.turn_45()}
    }
    pub fn turn_135(&mut self){
        for _ in 1..=3{ self.turn_45()}
    }
    pub fn turn_180(&mut self){
        for _ in 1..=4{ self.turn_45()}
    }
    pub fn turn_225(&mut self){
        for _ in 1..=5{ self.turn_45()}
    }
    pub fn turn_270(&mut self){
        for _ in 1..=6{ self.turn_45()}
    }
    pub fn turn_315(&mut self){
        for _ in 1..=7{ self.turn_45()}
    }

}