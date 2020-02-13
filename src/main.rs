use participant_lib::participant;
use termion::screen::AlternateScreen;
use std::{thread, time};
use std::io::{Write, stdout};
use termion::{cursor, clear};

fn main() {
    let mut screen = AlternateScreen::from(stdout());
    let (max_x,max_y):(u16,u16) = termion::terminal_size().unwrap();
    let mut part = participant::Participant::new(None,None,None,1,1,3,max_x as i32,max_y as i32);
    write!(screen,"{}{}",clear::All,cursor::Hide).unwrap();
    loop{
        let particle = part.get_particle();
        write!(screen,"{}{}{}",
            cursor::Goto(1,1),
            clear::CurrentLine,
            format!("Pos particle x:{} y:{} dir: {} {}",
                particle.get_pos_x(),
                particle.get_pos_y(),
                particle.get_sym(true),
                particle.get_dir().unwrap()
            )
        ).unwrap();
        write!(screen,"{}{}",
            cursor::Goto(particle.get_pos_x() as u16,particle.get_pos_y()as u16),
            particle.get_sym(false)
        ).unwrap();
        screen.flush().unwrap();
        thread::sleep(time::Duration::from_millis(10));
        write!(screen,"{}{}",
            cursor::Goto(particle.get_pos_x()as u16,particle.get_pos_y()as u16),
            ' '
        ).unwrap();
        part.particle_move();
    }
}
