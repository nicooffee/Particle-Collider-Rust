
#[allow(dead_code)]
pub mod source_list{
    pub mod source ;
    use source::Source;
    use source::limit_box::LimitBox;
    use source::particle::position::Position;
    pub struct SourceList{
        list_active:    Vec<Source>,
        list_nactive:   Vec<Source>
    }

    impl SourceList {
        pub fn new(cant_source: u32, cant_particle: u32, area: LimitBox) -> SourceList {
            let mut list:Vec<Source> = Vec::new();
            for _ in 0..cant_source{
                let (pos_x,pos_y) = area.get_rand_cord();
                list.push(
                    Source::new(
                        Some(pos_x), 
                        Some(pos_y), 
                        None, 
                        cant_particle,
                        area.get_lim_min().get_x(),
                        area.get_lim_min().get_y(),
                        area.get_lim_max().get_x(),
                        area.get_lim_max().get_y()
                    )
                );
            }
            SourceList {
                list_active: list,
                list_nactive: Vec::new()
            }
        }


        pub fn move_particle(&mut self, id_source: usize) -> Option<(Source,Position)> {
            if id_source < self.get_len_active(){
                let &pos = self.list_active[id_source].get_particle().get_pos();
                self.list_active[id_source].particle_move();
                match (self).get_collision(id_source) {
                    Some(x) => {
                        self.list_active[id_source].set_rand_pos();
                        while let Some(_) = self.get_collision(id_source){
                            
                            self.list_active[id_source].set_rand_pos();
                        }
                        self.list_active[id_source].sub_particle();
                        self.list_active[x].set_rand_pos();
                        while let Some(_) = self.get_collision(x){
                            self.list_active[x].set_rand_pos();
                        }
                        self.list_active[x].sub_particle();
                        Some((self.list_active[x],pos))
                    },
                    None => None
                }
            }
            else{
                None
            }
        }


        pub fn get_collision(&self,id: usize) -> Option<usize> {
            let mut coll = None;
            for i in 0..self.get_len_active(){
                if self.list_active[id].get_particle().comp_particle(self.list_active[i].get_particle()) &&
                    id != i
                    {
                        coll = Some(i);
                        break;
                    }
            }
            coll
        }

        
        pub fn get_len_active(&self) -> usize {
            self.list_active.len()
        }
        pub fn get_len_nactive(&self) -> usize {
            self.list_nactive.len()
        }

        pub fn get_source(&self,i: usize) -> Source {
            self.list_active[i]
        }
    }
}