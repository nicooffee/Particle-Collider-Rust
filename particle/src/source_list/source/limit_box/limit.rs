#[derive(Copy, Clone)]
pub struct Limit{
    lim_x: i32,
    lim_y: i32
}

impl Limit {
    pub fn new(lim_x: i32,lim_y: i32) -> Limit {
        Limit {
            lim_x: lim_x,
            lim_y: lim_y
        }
    }

    pub fn comp_x(&self,pos_x:i32,func: fn(a:i32,b:i32)->bool) -> bool{
        func(self.lim_x,pos_x)
    }
    pub fn comp_y(&self,pos_y:i32,func: fn(a:i32,b:i32)->bool) -> bool{
        func(self.lim_y,pos_y)
    }
    pub fn comp(&self,pos_x:i32,pos_y:i32,func: fn(a:i32,b:i32)->bool) -> bool{
        func(self.lim_x,pos_x) && func(self.lim_y,pos_y)
    }

    pub fn get_x(&self) -> i32 {
        self.lim_x
    }
    pub fn get_y(&self) -> i32 {
        self.lim_y
    }
}