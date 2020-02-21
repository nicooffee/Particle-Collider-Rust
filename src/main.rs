use participant_lib::source_list::SourceList;
use participant_lib::source_list::source::limit_box::LimitBox;
use participant_lib::source_list::source::particle::position::Position;
use termion::screen::AlternateScreen;
use std::{thread, time};
use std::sync::{Arc, Mutex};
use std::io::{Read, Write, stdout};
use termion::{cursor, clear};
use termion::raw::IntoRawMode;
use termion::async_stdin;
use termion_ext::AdvWrite;

const CANT_SOURCE: u32      = 3;
const CANT_PARTICLE: u32    = 2;


fn main() {
    let mut s_in = async_stdin().bytes();
    let mut s_out = AlternateScreen::from(stdout().into_raw_mode().unwrap());
    let (max_x,max_y):(u16,u16) = termion::terminal_size().unwrap();
    write!(s_out,"{}{}",clear::All,cursor::Hide).unwrap();
    s_out.w_box(1,2,max_x,max_y,None,None);
    s_out.flush().unwrap();
    let mut src_list = SourceList::new(CANT_SOURCE,CANT_PARTICLE,LimitBox::new(2,3,max_x as i32 -1,max_y as i32 -1));
    let src_list_share = Arc::new(Mutex::new(src_list));

    
}
