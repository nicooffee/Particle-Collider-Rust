use participant_lib::source_list::SourceList;
use participant_lib::source_list::source::limit_box::LimitBox;
use participant_lib::source_list::source::particle::position::Position;
use termion::screen::AlternateScreen;
use std::{thread, time};
use std::io::{Read, Write, stdout};
use termion::{cursor, clear,color};
use termion::raw::IntoRawMode;
use termion::async_stdin;
use termion_ext::AdvWrite;


fn main() {
    let mut s_in = async_stdin().bytes();
    let mut s_out = AlternateScreen::from(stdout().into_raw_mode().unwrap());
    let (max_x,max_y):(u16,u16) = termion::terminal_size().unwrap();
    write!(s_out,"{}{}",clear::All,cursor::Hide).unwrap();
    s_out.w_box(1,2,max_x,max_y,None,None);
    s_out.flush().unwrap();
    let mut source_list = SourceList::new(4,5,LimitBox::new(2,3,max_x as i32 -1,max_y as i32 -1));
    let mut it = 1;
    let mut c_col = 0;
    loop{
        let b = s_in.next();
        if let Some(Ok(b'q')) = b {
            break;
        }
        {
            s_out.w_go_str(1,1,
                format!("C_ACT: {}\tC_NACT: {}\tIT: {}\tIT%COL: {:.4}",
                    source_list.get_len_active(),
                    source_list.get_len_nactive(),
                    it,
                    (c_col as f32)/(it as f32)
                )
            );
        }
        for i in 0..source_list.get_len_active(){
            let opt = source_list.move_particle(i);
            let mut opt_col = None::<Position>;
            if let Some((coll,pos)) = opt {
                let p = coll.get_position();
                s_out.w_go_str(p.get_pos_x()as u16,p.get_pos_y()as u16,String::from(" "));
                opt_col = Some(pos);
                c_col = c_col + 1;
            }
            let src = source_list.get_source(i);
            let p_src = src.get_position();
            let p_prev_src = src.get_prev_position();
            s_out.w_go_str(p_prev_src.get_pos_x()as u16,p_prev_src.get_pos_y()as u16,String::from(" "));
            s_out.w_go_str(p_src.get_pos_x()as u16,p_src.get_pos_y()as u16,src.get_symbol(true).to_string());
            if let Some(pos_col) = opt_col{
                s_out.w_go_str(pos_col.get_pos_x()as u16,pos_col.get_pos_y()as u16,String::from("💥"));
            }
            it = it + 1;
        }
        s_out.flush().unwrap();
        thread::sleep(time::Duration::from_millis(10));
    }
}
