use participant_lib::source_list::SourceList;
use participant_lib::source_list::source::limit_box::LimitBox;
use termion::screen::AlternateScreen;
use std::{thread, time};
use std::sync::{Arc, Mutex};
use std::io::{Write, stdout,stdin};
use termion::{cursor, clear,color};
use termion::raw::IntoRawMode;
use termion::event::Key;
use termion::input::TermRead;
use termion_ext::AdvWrite;
use std::sync::mpsc::channel;

const CANT_SOURCE: usize      = 5;
const CANT_PARTICLE: u32    = 1000;
const DELAY:u64 = 1;

fn main() {
    let mut s_out = AlternateScreen::from(stdout().into_raw_mode().unwrap());
    let (max_x,max_y):(u16,u16) = termion::terminal_size().unwrap();
    write!(s_out,"{}{}",clear::All,cursor::Hide).unwrap();
    s_out.w_box(1,1,max_x,max_y,None,None);
    s_out.flush().unwrap();
    let src_list = SourceList::new(CANT_SOURCE,CANT_PARTICLE,LimitBox::new(2,2,max_x as i32 -1,max_y as i32 -1));
    let share_src_list  = Arc::new(Mutex::new(src_list));
    let share_s_out     = Arc::new(Mutex::new(s_out));
    let mut threads = vec![];
    let mut senders = vec![];
    for x in 0..CANT_SOURCE {
        let (sender,reciever) = channel();
        senders.push(sender);
        threads.push(
            thread::spawn({
                    let clone_src_list  = Arc::clone(&share_src_list);
                    let clone_s_out     = Arc::clone(&share_s_out);
                    move || {
                        source_run(x, clone_src_list, clone_s_out,reciever);
                    }
                }
            )
            
        );
    }
    thread::spawn(move || {
        loop{
            let s_in = stdin();
            for c in s_in.keys() {
                match c.unwrap() {
                    Key::Char('q') =>  {
                        for i in 0..CANT_SOURCE{
                            senders[i].send(true).unwrap();
                        }
                        break;
                    },
                    _              => (),
                }
            }
        }
    }); 
    for t in threads {
        t.join().unwrap();
    }
}


fn source_run<W: std::io::Write>(
    x: usize,
    clone_src_list: Arc<Mutex<SourceList>>,
    clone_s_out: Arc<Mutex<AlternateScreen<W>>>,
    exit_msg: std::sync::mpsc::Receiver<bool>){
    loop {
        match exit_msg.try_recv() {Ok(_b) => break, _ => ()};
        thread::sleep(time::Duration::from_micros(DELAY));
        let mut src_l = clone_src_list.lock().unwrap();
        let mut s_out = clone_s_out.lock().unwrap();
        match src_l.get_source_act(x){
            None => break,
            Some(_) => {
                if let Some((coll,pos)) = src_l.move_particle(x){
                    coll.particle_clear(&mut s_out);
                    write!(s_out,"{}{} ",cursor::Goto(pos.get_pos_x()as u16,pos.get_pos_y()as u16),color::Fg(color::Red)).unwrap();
                    s_out.w_go_str(pos.get_pos_x()as u16,pos.get_pos_y()as u16,String::from(" "));
                    write!(s_out,"{}",color::Fg(color::Reset)).unwrap();
                }
                match src_l.check_src(x) {
                    false => if let Some(src)=src_l.get_source_act(x){src.particle_clear(&mut s_out);},
                    true => if let Some(src)=src_l.get_source_act(x){src.particle_print(&mut s_out,false);}
                }
                s_out.flush().unwrap();
            }
        }
    }
}