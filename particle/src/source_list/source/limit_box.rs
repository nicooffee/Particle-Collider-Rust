pub mod limit;
use rand::distributions::{Distribution, Uniform};

#[derive(Copy,Clone)]
pub struct LimitBox {
    lim_min: limit::Limit,
    lim_max: limit::Limit
}

pub enum Area{
    Inside,
    OutSide1,
    OutSide2,
    OutSide3,
    OutSide4,
    OutCorner1,
    OutCorner2,
    OutCorner3,
    OutCorner4
}

impl LimitBox {
    pub fn new(min_x: i32,min_y: i32,max_x:i32,max_y:i32) -> LimitBox {
        LimitBox{
            lim_min : limit::Limit::new(min_x,min_y),
            lim_max : limit::Limit::new(max_x,max_y)
        }
    }


    pub fn area_point(&self,pos_x:i32,pos_y:i32) ->Result<Area,&str>{
        let caso_lim:(bool,bool,bool,bool) = (
            self.lim_min.comp_x(pos_x,|a,b| a<=b),
            self.lim_min.comp_y(pos_y,|a,b| a<=b),
            self.lim_max.comp_x(pos_x,|a,b| a>=b),
            self.lim_max.comp_y(pos_y,|a,b| a>=b)
        );
        match caso_lim{
            (true,true,false,true) => Ok(Area::OutSide1),
            (true,false,true,true) => Ok(Area::OutSide2),
            (false,true,true,true) => Ok(Area::OutSide3),
            (true,true,true,false) => Ok(Area::OutSide4),
            (true,false,false,true) => Ok(Area::OutCorner1),
            (false,false,true,true) => Ok(Area::OutCorner2),
            (false,true,true,false) => Ok(Area::OutCorner3),
            (true,true,false,false) => Ok(Area::OutCorner4),
            (true,true,true,true) => Ok(Area::Inside),
            _ => Err("la wea mala")
        }
    }

    pub fn get_lim_min(&self) -> limit::Limit {
        self.lim_min
    }

    pub fn get_lim_max(&self) -> limit::Limit {
        self.lim_max
    }

    pub fn get_min_x(&self) -> i32 {
        self.lim_min.get_x()
    }
    pub fn get_min_y(&self) -> i32 {
        self.lim_min.get_y()
    }
    pub fn get_max_x(&self) -> i32 {
        self.lim_max.get_x()
    }
    pub fn get_max_y(&self) -> i32 {
        self.lim_max.get_y()
    }

    pub fn get_rand_cord(&self) -> (i32,i32){
        let mut rng = rand::thread_rng();
        let un_x = Uniform::new(self.lim_min.get_x()+1,self.lim_max.get_x()-1);
        let un_y = Uniform::new(self.lim_min.get_y()+1,self.lim_max.get_y()-1);
        
        (
            un_x.sample(&mut rng),
            un_y.sample(&mut rng)
        )
    }
}