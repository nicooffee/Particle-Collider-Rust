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
const CANT_PARTICLE: u32    = 10000;
const DELAY:u64 = 10000;
fn main() {
    let mut s_out = AlternateScreen::from(stdout().into_raw_mode().unwrap());
    let (limits_info,limits_srce,limits_heat) = initialize_window(&mut s_out);
    let src_list = SourceList::new(CANT_SOURCE,CANT_PARTICLE,limits_srce);
    let share_src_list  = Arc::new(Mutex::new(src_list));
    let share_s_out     = Arc::new(Mutex::new(s_out));
    let mut threads = vec![];
    let mut senders = vec![];
    let (sender_heat,reciever_heat) = channel();
    for x in 0..CANT_SOURCE + 1 {
        let (sender,reciever) = channel();
        senders.push(sender);
        threads.push(
                thread::spawn({
                    let clone_src_list  = Arc::clone(&share_src_list);
                    let clone_s_out     = Arc::clone(&share_s_out);
                    let sender_heat_clone = sender_heat.clone();
                    move || {
                        source_run(x, clone_src_list, clone_s_out,sender_heat_clone,reciever);
        }}));
    }
    let (sender_info,reciever_info) = channel();
    senders.push(sender_info);
    let (sender_heat_exit,reciever_heat_exit) = channel();
    senders.push(sender_heat_exit);
    threads.push(thread::spawn({
        let clone_s_out   = Arc::clone(&share_s_out);
        move || {
            heat_map_run(limits_heat, clone_s_out, reciever_heat,reciever_heat_exit)
    }}));
    threads.push(thread::spawn({
        let clone_src_list  = Arc::clone(&share_src_list);
        let clone_s_out     = Arc::clone(&share_s_out);
        move || {
            info_run(clone_src_list,clone_s_out,limits_info,reciever_info);
    }}));

    threads.push(thread::spawn(move || {
        exit_run(senders,sender_heat.clone());
    }));
    for t in threads {
        t.join().unwrap();
    }
}

fn source_run<W: std::io::Write>(
    x: usize,
    clone_src_list: Arc<Mutex<SourceList>>,
    clone_s_out: Arc<Mutex<AlternateScreen<W>>>,
    send_pos_heat: std::sync::mpsc::Sender<Option<(usize,usize)>>,
    exit_msg: std::sync::mpsc::Receiver<bool>){
    loop {
        match exit_msg.try_recv() {Ok(_b) => break, _ => ()};
        thread::sleep(time::Duration::from_micros(DELAY));
        let mut s_out = clone_s_out.lock().unwrap();
        let mut src_l = clone_src_list.lock().unwrap();
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
                    let tr_x:usize = (pos.get_pos_x() - min_x) as usize;
                    let tr_y:usize = (pos.get_pos_y() - min_y) as usize;
                    send_pos_heat.send(Some((tr_x,tr_y))).unwrap();
                }
                match src_l.check_src(x) {
                    false => if let Some(src)=src_l.get_source_act(x){src.particle_clear(&mut s_out);},
                    true => if let Some(src)=src_l.get_source_act(x){src.particle_print(&mut s_out,true);}
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
        let mut s_out = clone_s_out.lock().unwrap();
        let mut src_l = clone_src_list.lock().unwrap();
        for i in 0..src_l.get_len_active(){
            if let Some(src) = src_l.get_source_act(i){
                let pos_y:u16 = i as u16 + 2;
                if let Ok(_) = write!(s_out,"{}P {:2}-{}: {:5} | {} | ({:3},{:3})",
                    cursor::Goto(limits.get_min_x() as u16,pos_y),
                    i,
                    src.get_id(),
                    src.get_c_particle(),
                    src.get_symbol(true),
                    src.get_position().get_pos_x()-src.get_min_x()+1,
                    src.get_position().get_pos_y()-src.get_min_y()+1
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
    limits_heat: LimitBox,
    clone_s_out: Arc<Mutex<AlternateScreen<W>>>,
    pos_rec: std::sync::mpsc::Receiver<Option<(usize,usize)>>,
    exit_msg: std::sync::mpsc::Receiver<bool>){
    let mut heat_m = vec![vec![0;(limits_heat.get_max_y()-limits_heat.get_min_y()+1) as usize];(limits_heat.get_max_x()-limits_heat.get_min_x()+1) as usize];
    let mut max_col = 0;
    loop {
        match exit_msg.try_recv() {Ok(_b) => break, _ => ()};
        thread::sleep(time::Duration::from_micros(DELAY));
        if let Some((x,y)) = pos_rec.recv().unwrap(){
            let mut s_out   = clone_s_out.lock().unwrap();
            heat_m[x][y] = heat_m[x][y] + 1;
            if heat_m[x][y]> max_col {max_col = heat_m[x][y];
                for i in 0..heat_m.len(){
                    for j in 0..heat_m[i].len(){
                        let (r,g,b):(u8,u8,u8) = get_heat(heat_m[i][j],max_col);
                        s_out.w_go_str_color(limits_heat.get_min_x() as u16+i as u16,limits_heat.get_min_y() as u16+j as u16," ".to_string(),color::Reset,color::Rgb(r,g,b));
                    }
                }
            }
            let (r,g,b):(u8,u8,u8) = get_heat(heat_m[x][y],max_col);
            s_out.w_go_str_color(limits_heat.get_min_x() as u16+x as u16,limits_heat.get_min_y() as u16+y as u16," ".to_string(),color::Reset,color::Rgb(r,g,b));
        } 
    }
}

fn get_heat(c_val: i32, max_val: i32) -> (u8,u8,u8){
    let grd = ((c_val as f32 / max_val as f32) * 1020.0) as u32;
    match grd {
        766..=1020  => (255,(1020-grd) as u8,0),
        511..=765   => ((grd - 510) as u8,255,0),
        256..=510   => (0,255,(510-grd) as u8),
        _           => (0,grd as u8,255),
    }    
}

fn exit_run(
    senders: Vec<std::sync::mpsc::Sender<bool>>,
    unlock_heat_run: std::sync::mpsc::Sender<Option<(usize,usize)>>){
    let mut b_flag = false;
    loop{
        let s_in = stdin();
        for c in s_in.keys() {
            match c.unwrap() {
                Key::Char('q') =>  {
                    for i in 0..senders.len(){
                        if let Ok(_) = senders[i].send(true){}
                    }
                    if let Ok(_) = unlock_heat_run.send(None){}
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

