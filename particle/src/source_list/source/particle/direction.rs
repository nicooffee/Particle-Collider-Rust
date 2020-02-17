use rand::distributions::{Distribution, Uniform};
#[derive(Copy, Clone)]
pub struct Direction {
    dir_x: i8,
    dir_y: i8
}
#[derive(Debug)]
pub enum Dir{
    D0,
    D45,
    D90,
    D135,
    D180,
    D225,
    D270,
    D315
}

impl Dir {
    pub fn rand() -> Dir {
        let mut rng = rand::thread_rng();
        let uni = Uniform::from(0..8);
        match uni.sample(&mut rng) {
            0 => Dir::D0,
            1 => Dir::D45,
            2 => Dir::D90,
            3 => Dir::D135,
            4 => Dir::D180,
            5 => Dir::D225,
            6 => Dir::D270,
            7 => Dir::D315,
            _ => panic!()
        }
    }
}

impl Direction {
    pub fn new() -> Direction{
        Direction{
            dir_x: 1,
            dir_y: 0
        }
    }
    pub fn get_dir(&self) -> Result<Dir,&str>{
        match self{
            Direction{dir_x:1,dir_y:0}      => Ok(Dir::D0),
            Direction{dir_x:1,dir_y:-1}     => Ok(Dir::D45),
            Direction{dir_x:0,dir_y:-1}     => Ok(Dir::D90),
            Direction{dir_x:-1,dir_y:-1}    => Ok(Dir::D135),
            Direction{dir_x:-1,dir_y:0}     => Ok(Dir::D180),
            Direction{dir_x:-1,dir_y:1}     => Ok(Dir::D225),
            Direction{dir_x:0,dir_y:1}      => Ok(Dir::D270),
            Direction{dir_x:1,dir_y:1}      => Ok(Dir::D315),
            _ => Err("Direccion no definida")
        }
    }
    pub fn change_to(&mut self,dir: Dir){
        match dir {
            Dir::D0     => {self.dir_x = 1; self.dir_y = 0;},
            Dir::D45    => {self.dir_x = 1; self.dir_y = -1;},
            Dir::D90    => {self.dir_x = 0; self.dir_y = -1;},
            Dir::D135   => {self.dir_x = -1; self.dir_y = -1;},
            Dir::D180   => {self.dir_x = -1; self.dir_y = 0;},
            Dir::D225   => {self.dir_x = -1; self.dir_y = 1;},
            Dir::D270   => {self.dir_x = 0; self.dir_y = 1;},
            Dir::D315   => {self.dir_x = 1; self.dir_y = 1;}
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

    fn turn_45(&mut self){
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

    pub fn turn(&mut self,dir: Dir){
        match dir{
            Dir::D0     => (),
            Dir::D45    => self.turn_45(),
            Dir::D90    => for _ in 1..=2{ self.turn_45()},
            Dir::D135   => for _ in 1..=3{ self.turn_45()},
            Dir::D180   => for _ in 1..=4{ self.turn_45()},
            Dir::D225   => for _ in 1..=5{ self.turn_45()},
            Dir::D270   => for _ in 1..=6{ self.turn_45()},
            Dir::D315   => for _ in 1..=7{ self.turn_45()}
        }
    }

}