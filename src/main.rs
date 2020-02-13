use participant_lib::participant::particle;
use termion::screen::AlternateScreen;
use std::{thread, time};
use std::io::{Write, stdout};
use termion::{cursor, clear};

fn main() {
    let mut screen = AlternateScreen::from(stdout());
    let (max_x,max_y):(u16,u16) = termion::terminal_size().unwrap();
    let mut part = particle::Particle::new((max_x as i32)/2 ,(max_y as i32)/2,'A');
    write!(screen,"{}{}",clear::All,cursor::Hide).unwrap();
    loop{
        write!(screen,"{}{}",
            cursor::Goto(part.get_pos_x() as u16,part.get_pos_y()as u16),
            part.get_sym(false)
        ).unwrap();
        screen.flush().unwrap();
        thread::sleep(time::Duration::from_millis(100));
        write!(screen,"{}{}",
            cursor::Goto(part.get_pos_x()as u16,part.get_pos_y()as u16),
            ' '
        ).unwrap();
        part.par_move();

        part.move_45();
    }
}
