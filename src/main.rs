use participant_lib::source_list::SourceList;
use participant_lib::source_list::source::limit_box::LimitBox;
use termion::screen::AlternateScreen;
use std::{thread, time};
use std::sync::{Arc, Mutex};
use std::io::{Read, Write, stdout};
use termion::{cursor, clear};
use termion::raw::IntoRawMode;
use termion::async_stdin;
use termion_ext::AdvWrite;

const CANT_SOURCE: usize      = 5;
const CANT_PARTICLE: u32    = 2;


fn main() {
    let mut s_in = async_stdin().bytes();
    let mut s_out = AlternateScreen::from(stdout().into_raw_mode().unwrap());
    let (max_x,max_y):(u16,u16) = termion::terminal_size().unwrap();
    write!(s_out,"{}{}",clear::All,cursor::Hide).unwrap();
    s_out.w_box(1,2,max_x,max_y,None,None);
    s_out.flush().unwrap();
    let src_list = SourceList::new(CANT_SOURCE,CANT_PARTICLE,LimitBox::new(2,3,max_x as i32 -1,max_y as i32 -1));
    
    let share_src_list  = Arc::new(Mutex::new(src_list));
    let share_s_out     = Arc::new(Mutex::new(s_out));
    let share_s_in      = Arc::new(Mutex::new(s_in));

    let mut threads = vec![];
    for x in 0..CANT_SOURCE {
        threads.push(
            thread::spawn({
                    let clone_src_list  = Arc::clone(&share_src_list);
                    let clone_s_out     = Arc::clone(&share_s_out);
                    let clone_s_in      = Arc::clone(&share_s_in);
                    move || { 
                        loop {
                            thread::sleep(time::Duration::from_millis(20));
                            let mut src_l = clone_src_list.lock().unwrap();
                            let mut s_out = clone_s_out.lock().unwrap();
                            match src_l.get_source_act(x){
                                None => break,
                                Some(_) => {
                                    src_l.move_particle(x);
                                    src_l.get_source_act(x).unwrap().particle_print(&mut s_out,true);
                                    s_out.flush().unwrap();
                                }
                            }
                        }
                    }
                }
            )
        );
    }
    for t in threads {
        t.join().unwrap();
    }


    thread::sleep(time::Duration::from_millis(2000));
}
