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

    pub fn move_45(&mut self){
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
    pub fn move_90(&mut self){
        for _ in 1..=2{ self.move_45()}
    }
    pub fn move_135(&mut self){
        for _ in 1..=3{ self.move_45()}
    }
    pub fn move_180(&mut self){
        for _ in 1..=4{ self.move_45()}
    }
    pub fn move_225(&mut self){
        for _ in 1..=5{ self.move_45()}
    }
    pub fn move_270(&mut self){
        for _ in 1..=6{ self.move_45()}
    }
    pub fn move_315(&mut self){
        for _ in 1..=7{ self.move_45()}
    }

}