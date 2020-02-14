use participant_lib::participant;
use termion::screen::AlternateScreen;
use std::{thread, time};
use std::io::{Read, Write, stdout};
use termion::{cursor, clear};
use termion::raw::IntoRawMode;
use termion::async_stdin;
use termion_ext::AdvWrite;


fn main() {
    let mut s_in = async_stdin().bytes();
    let mut s_out = AlternateScreen::from(stdout().into_raw_mode().unwrap());
    let (max_x,max_y):(u16,u16) = termion::terminal_size().unwrap();
    let mut part = participant::Participant::new(None,None,None,2,2,3,max_x as i32-1,max_y as i32-1);
    write!(s_out,"{}{}",clear::All,cursor::Hide).unwrap();
    s_out.w_box(1,2,max_x,max_y,'*');
    loop{
        let b = s_in.next();
        if let Some(Ok(b'q')) = b {
            break;
        }
        let particle = part.get_particle();
        write!(s_out,"{}{}{}",
            cursor::Goto(1,1),
            clear::CurrentLine,
            format!("max: {} {} Pos particle x:{:3} y:{:3}\tdir: {} {:?}",max_x,max_y,
                particle.get_pos_x(),
                particle.get_pos_y(),
                particle.get_sym(true),
                particle.get_dir().unwrap()
            )
        ).unwrap();
        write!(s_out,"{}{}",
            cursor::Goto(particle.get_pos_x() as u16,particle.get_pos_y()as u16),
            particle.get_sym(false)
        ).unwrap();
        s_out.flush().unwrap();
        thread::sleep(time::Duration::from_millis(10));
        write!(s_out,"{}{}",
            cursor::Goto(particle.get_pos_x()as u16,particle.get_pos_y()as u16),
            ' '
        ).unwrap();
        part.particle_move();
    }
}
