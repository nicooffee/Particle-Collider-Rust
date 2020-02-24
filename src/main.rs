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

const CANT_SOURCE: usize      = 10;
const CANT_PARTICLE: u32    = 100;
const DELAY:u64 = 100;
fn main() {
    let mut s_out = AlternateScreen::from(stdout().into_raw_mode().unwrap());
    let (limits_info,limits_srce,limits_heat) = initialize_window(&mut s_out);
    let src_list = SourceList::new(CANT_SOURCE,CANT_PARTICLE,limits_srce);
    let heat_m = vec![vec![0;(limits_heat.get_max_y()-limits_heat.get_min_y()+1) as usize];(limits_heat.get_max_x()-limits_heat.get_min_x()+1) as usize];
    let share_src_list  = Arc::new(Mutex::new(src_list));
    let share_s_out     = Arc::new(Mutex::new(s_out));
    let share_heat_m    = Arc::new(Mutex::new(heat_m));
     let mut threads = vec![];
    let mut senders = vec![];
    for x in 0..CANT_SOURCE + 1 {
        let (sender,reciever) = channel();
        senders.push(sender);
        threads.push(
                thread::spawn({
                    let clone_src_list  = Arc::clone(&share_src_list);
                    let clone_s_out     = Arc::clone(&share_s_out);
                    let clone_heat_m  = Arc::clone(&share_heat_m);
                    move || {
                        source_run(x, clone_src_list, clone_s_out,clone_heat_m,reciever);
        }}));
    }
    let (sender,reciever) = channel();
    senders.push(sender);
    threads.push(thread::spawn({
        let clone_src_list  = Arc::clone(&share_src_list);
        let clone_s_out     = Arc::clone(&share_s_out);
        move || {
            info_run(clone_src_list,clone_s_out,limits_info,reciever);
    }}));
    let (sender,reciever) = channel();
    senders.push(sender);
    threads.push(thread::spawn({
        let clone_heat_m  = Arc::clone(&share_heat_m);
        let clone_s_out   = Arc::clone(&share_s_out);
        move || {
            heat_map_run(limits_heat.get_min_x(),limits_heat.get_min_y(), clone_s_out, clone_heat_m, reciever)
    }}));

    threads.push(thread::spawn(move || {
        exit_run(senders);
    }));
    for t in threads {
        t.join().unwrap();
    }
}

fn source_run<W: std::io::Write>(
    x: usize,
    clone_src_list: Arc<Mutex<SourceList>>,
    clone_s_out: Arc<Mutex<AlternateScreen<W>>>,
    clone_heat_m: Arc<Mutex<Vec<Vec<i32>>>>,
    exit_msg: std::sync::mpsc::Receiver<bool>){
    loop {
        match exit_msg.try_recv() {Ok(_b) => break, _ => ()};
        thread::sleep(time::Duration::from_micros(DELAY));
        let mut src_l = clone_src_list.lock().unwrap();
        let mut s_out = clone_s_out.lock().unwrap();
        match src_l.get_source_act(x){
            None => break,
            Some(_) => {
                let min_x = src_l.get_source_act(x).unwrap().get_min_x();
                let min_y = src_l.get_source_act(x).unwrap().get_min_y();
                if let Some((coll,pos)) = src_l.move_particle(x){
                    coll.particle_clear(&mut s_out);
                    write!(s_out,"{}{} ",cursor::Goto(pos.get_pos_x()as u16,pos.get_pos_y()as u16),color::Fg(color::Red)).unwrap();
                    s_out.w_go_str(pos.get_pos_x()as u16,pos.get_pos_y()as u16,String::from("X"));
                    write!(s_out,"{}",color::Fg(color::Reset)).unwrap();
                    let mut heat_m = clone_heat_m.lock().unwrap();
                    let tr_x:usize = (pos.get_pos_x() - min_x) as usize;
                    let tr_y:usize = (pos.get_pos_y() - min_y) as usize;
                    heat_m[tr_y][tr_x] = heat_m[tr_y][tr_x] + 1 ; 
                }
                match src_l.check_src(x) {
                    false => if let Some(src)=src_l.get_source_act(x){src.particle_clear(&mut s_out);},
                    true => if let Some(src)=src_l.get_source_act(x){src.particle_print(&mut s_out,false);}
                }
            }
        }
    }
}

fn info_run<W: std::io::Write>(
    clone_src_list: Arc<Mutex<SourceList>>,
    clone_s_out: Arc<Mutex<AlternateScreen<W>>>,
    limits: LimitBox,
    exit_msg: std::sync::mpsc::Receiver<bool>){
    let max_l_bar:u16 = 35;
    let max_string = (0..max_l_bar).map(|_| "|").collect::<String>();
    loop {
        match exit_msg.try_recv() {Ok(_b) => break, _ => ()};
        thread::sleep(time::Duration::from_micros(DELAY));
        let mut src_l = clone_src_list.lock().unwrap();
        let mut s_out = clone_s_out.lock().unwrap();
        for i in 0..src_l.get_len_active(){
            if let Some(src) = src_l.get_source_act(i){
                let pos_y:u16 = i as u16 + 2;
                if let Ok(_) = write!(s_out,"{}P {:2}-{}: {:5} | {} | ({:3},{:3})",
                    cursor::Goto(limits.get_min_x() as u16,pos_y),
                    i,
                    src.get_id(),
                    src.get_c_particle(),
                    src.get_symbol(true),
                    src.get_position().get_pos_x(),
                    src.get_position().get_pos_y()
                ){};
                let percent = src.get_c_particle() as f32/CANT_PARTICLE as f32;
                let lim_max = (percent*(max_l_bar-6) as f32) as usize ;
                if let Ok(_) = write!(s_out,"{}{}{:3.1}% {}{} {}",
                    cursor::Goto(limits.get_max_x() as u16-(max_l_bar),pos_y ),
                    color::Fg(get_bar_color(percent*100.0)),
                    percent*100.0,
                    cursor::Goto(limits.get_max_x() as u16-(max_l_bar) + 6,pos_y),
                    &max_string[0..lim_max],
                    color::Fg(color::Reset)
                ){};
            }
        }
        s_out.w_line_h(limits.get_min_x()as u16,(src_l.get_len_active()+2) as u16,(limits.get_max_x()-limits.get_min_x()) as u16,' ');
        s_out.flush().unwrap();
    }        
}
fn get_bar_color(l_bar: f32) -> color::Rgb{
    match l_bar as u32 {
        96..=100 => color::Rgb(0,255,255),
        86..=95 => color::Rgb(0,255,0),
        51..=85 => color::Rgb(255,255,0),
        25..=50 => color::Rgb(255, 165, 0),
        _ => color::Rgb(255,0,0)

    }
}

fn heat_map_run<W: std::io::Write>(
    tr_x: i32,
    tr_y: i32,
    clone_s_out: Arc<Mutex<AlternateScreen<W>>>,
    clone_heat_m: Arc<Mutex<Vec<Vec<i32>>>>,
    exit_msg: std::sync::mpsc::Receiver<bool>){
    loop {
        match exit_msg.try_recv() {Ok(_b) => break, _ => ()};
        thread::sleep(time::Duration::from_micros(DELAY));
        let heat_m  = clone_heat_m.lock().unwrap();
        for i in 0..heat_m.len(){
            for j in 0..heat_m[i].len(){
                if heat_m[i][j] > 0 {
                    let grd:u8 = if heat_m[i][j]> 255 {255} else {heat_m[i][j] as u8};
                    let mut s_out   = clone_s_out.lock().unwrap();
                    s_out.w_go_str_color(tr_x as u16+i as u16,tr_y as u16+j as u16," ".to_string(),color::Reset,color::Rgb(grd,grd,grd));
                }
            }
        }
    }
    
}

fn exit_run(senders: Vec<std::sync::mpsc::Sender<bool>>){
    let mut b_flag = false;
    loop{
        let s_in = stdin();
        for c in s_in.keys() {
            match c.unwrap() {
                Key::Char('q') =>  {
                    for i in 0..senders.len(){
                        if let Ok(_) = senders[i].send(true){}
                    }
                    b_flag = true;
                    break;
                },
                _              => (),
            }
        }
        if b_flag {break;}
    }
}

fn initialize_window<W: std::io::Write>(s_out: &mut AlternateScreen<W>) -> (LimitBox,LimitBox,LimitBox){
    let (max_x,max_y):(u16,u16) = termion::terminal_size().unwrap();
    let (min_bi_x,min_bi_y,max_bi_x,max_bi_y) = (1,1,max_x*40/100,max_y);
    let (min_bh_x,min_bh_y,max_bh_x,max_bh_y) = (max_bi_x+1,1,max_bi_x+1+(max_x-max_bi_x+1)/2,max_y);
    let (min_bs_x,min_bs_y,max_bs_x,max_bs_y) = (max_bh_x+1,1,max_x,max_y);
    let limits_info = LimitBox::new(min_bi_x+1 ,min_bi_y+1,(max_bi_x as i32)-1 ,(max_bi_y as i32)-1);
    let limits_srce = LimitBox::new((min_bs_x as i32)+1,min_bs_y+1,(max_bs_x as i32)-1,(max_bs_y as i32)-1);
    let limits_heat = LimitBox::new((min_bh_x as i32) + 1,min_bh_y+1,(max_bh_x as i32)-1,(max_bh_y as i32)-1);
    write!(s_out,"{}{}",clear::All,cursor::Hide).unwrap();
    s_out.w_box(min_bi_x as u16,min_bi_y as u16,max_bi_x,max_bi_y,None,None);
    s_out.w_box(min_bh_x,min_bh_y as u16,max_bh_x,max_bh_y,None,None);
    s_out.w_box(min_bs_x,min_bs_y as u16,max_bs_x,max_bs_y,None,None);
    s_out.flush().unwrap();
    (limits_info,limits_srce,limits_heat)
}

