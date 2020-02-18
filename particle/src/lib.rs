
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
                        SourceList::gen_id(list.len() as u32),
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

        fn gen_id(c_src: u32) -> String {
            let root =  String::from("AAAA");
            let mut exp:u32 = 26u32.pow(root.len() as u32);
            let mut c_src = c_src;
            root.chars().map(|x| {
                exp = exp/26;
                let r: u8 = (c_src / exp) as u8;
                let new_c = ( x as u8 + r) as char;
                if r>0 {c_src=c_src-exp*(r as u32)}
                new_c
            }).collect()
        }

        pub fn move_particle(&mut self, id_source: usize) -> Option<(&Source,Position)> {
            if id_source < self.get_len_active(){
                let &pos = self.list_active[id_source].get_position();
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
                        Some((&self.list_active[x],pos))
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
                if self.list_active[id].comp_particle(&self.list_active[i]) &&
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

        pub fn get_source(&self,i: usize) -> &Source {
            &self.list_active[i]
        }

    }

    #[cfg(test)]
    mod tests {
        use super::SourceList;

        #[test]
        fn id() {
            assert_eq!(String::from("AAAC"),SourceList::gen_id(2));
            assert_eq!(String::from("AABA"),SourceList::gen_id(26));
            assert_eq!(String::from("ABAA"),SourceList::gen_id(676));
            assert_eq!(String::from("BAAA"),SourceList::gen_id(17576));
            assert_eq!(String::from("ZZZZ"),SourceList::gen_id(456975));
        }
    }
}

