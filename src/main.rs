use participant_lib::source_list::SourceList;
use participant_lib::source_list::source::limit_box::LimitBox;
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
    s_out.w_box(1,1,max_x,max_y,None,None);
    s_out.flush().unwrap();
    let mut source_list = SourceList::new(4,5,
        LimitBox::new(2,2,max_x as i32 -1,max_y as i32 -1));
    loop{
        let b = s_in.next();
        if let Some(Ok(b'q')) = b {
            break;
        }
        for i in 0..source_list.get_len_active(){
            {
                let s = source_list.get_source(i);
                s_out.w_go_str(
                    s.get_particle().get_pos_x() as u16,
                    s.get_particle().get_pos_y() as u16,
                    String::from(" ")
                );
            }
            if let Some((_,pos)) = source_list.move_particle(i){
                s_out.w_go_str(pos.get_pos_x() as u16,pos.get_pos_y() as u16,String::from("💥"))
            }
            {
                let s = source_list.get_source(i);
                s_out.w_go_str(
                    s.get_particle().get_pos_x() as u16,
                    s.get_particle().get_pos_y() as u16,
                    s.get_particle().get_sym(true).to_string()
                );
            }
        }
        s_out.flush().unwrap();
        thread::sleep(time::Duration::from_millis(10));
    }
}
