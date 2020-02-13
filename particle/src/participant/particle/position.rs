#[derive(Copy, Clone)]
pub struct Position {
    pos_x:i32,
    pos_y:i32
}

impl Position {
    pub fn new(pos_x: i32,pos_y: i32) -> Position {
        Position {
            pos_x : pos_x,
            pos_y : pos_y
        }
    }
    pub fn get_pos_x(&self) -> i32{
        self.pos_x
    }
    pub fn get_pos_y(&self) -> i32{
        self.pos_y
    }
    pub fn set_pos_x(&mut self,n_pos_x: i32){
        self.pos_x = n_pos_x;
    }
    pub fn set_pos_y(&mut self,n_pos_y: i32){
        self.pos_y =n_pos_y;
    }
}