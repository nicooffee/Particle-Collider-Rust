pub mod limit;
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


    pub fn area_point(&self,pos_x:i32,pos_y:i32) ->Area{
        let caso_lim:(bool,bool,bool,bool) = (
            self.lim_min.comp_x(pos_x,|a,b| a<=b),
            self.lim_min.comp_y(pos_y,|a,b| a<=b),
            self.lim_max.comp_x(pos_x,|a,b| a>=b),
            self.lim_max.comp_y(pos_y,|a,b| a>=b)
        );
        match caso_lim{
            (true,true,false,true) => Area::OutSide1,
            (true,false,true,true) => Area::OutSide2,
            (false,true,true,true) => Area::OutSide3,
            (true,true,true,false) => Area::OutSide4,
            (true,false,false,true) => Area::OutCorner1,
            (false,false,true,true) => Area::OutCorner2,
            (false,true,true,false) => Area::OutCorner3,
            (true,true,false,false) => Area::OutCorner4,
            _ => Area::Inside
        }

    }
}