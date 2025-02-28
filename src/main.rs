
use std::{thread, time};
use rand::Rng;

struct Cleanup;

impl Drop for Cleanup
{
    fn drop(&mut self)
    {
        ncurses::endwin();
        println!("Cleaning up");
    }
}

#[derive(Clone)]
struct Droplet {
    length : i32,
    col : i32,
    row : i32,
    frames_per_row : i32
}

impl Droplet
{
    fn new(max_row : i32, max_col : i32) -> Droplet {
        Droplet {
            length : rand::rng().random_range(2..max_row/4),
            col : rand::rng().random_range(0..max_col),
            row : -rand::rng().random_range(0..max_row),
            frames_per_row : 0
        }
    }
}


fn main() {
    let _cleanup = Cleanup;
    let tick_rate = time::Duration::from_millis(20);

    let mut max_row : i32 = 100;
    let mut max_col : i32 = 100;

    // ncurses::initscr();
    // ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    // ncurses::keypad(ncurses::stdscr(), true);
    // ncurses::noecho();
    // ncurses::cbreak();
    // ncurses::nodelay(ncurses::stdscr(), true);

    // ncurses::getmaxyx(ncurses::stdscr(), &mut max_row, &mut max_col);

    // ncurses::start_color();
    // ncurses::use_default_colors();
    // ncurses::init_pair(1, ncurses::COLOR_BLACK, -1);
    // ncurses::init_pair(2, ncurses::COLOR_RED, -1); 
    // ncurses::init_pair(3, ncurses::COLOR_GREEN, -1);
    // ncurses::init_pair(4, ncurses::COLOR_YELLOW, -1);
    // ncurses::init_pair(5, ncurses::COLOR_BLUE, -1); 
    // ncurses::init_pair(6, ncurses::COLOR_MAGENTA, -1);
    // ncurses::init_pair(7, ncurses::COLOR_CYAN, -1); 
    // ncurses::init_pair(8, ncurses::COLOR_WHITE, -1);

    let screen_area : u32 = (max_row as u32) * (max_col as u32);
    let n_droplets : u32 = screen_area/50;
    let mut droplets : Vec<Droplet> = Vec::new();
    
    for _ in 1..=n_droplets {
        droplets.push(Droplet::new(max_row, max_col));
    }

    for drop in droplets
    {
        println!("{0}, {1}, {2}, {3}", drop.length, drop.col, drop.row, drop.frames_per_row);
    }

    // loop
    // {

    //     thread::sleep(tick_rate);

    //     break;
    // }

    // ncurses::endwin();
}
