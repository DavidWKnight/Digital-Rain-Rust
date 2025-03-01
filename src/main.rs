
use std::{thread, time};
use rand::Rng;

const MIN_CHAR : u32 = 33;
const MAX_CHAR : u32 = 126;

struct Cleanup;

impl Drop for Cleanup
{
    fn drop(&mut self)
    {
        ncurses::endwin();
        println!("Cleaning up");
    }
}

#[derive(Copy, Clone)]
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

#[derive(Copy, Clone)]
struct DrawSettings {
    max_row : i32,
    max_col : i32,

    refresh_rate : i32,
    multicolor_trail : bool,
    primary_color : i32,
    secondary_color : i32,
    go_fast : bool,
}


fn draw_drop(drop : &mut Droplet, draw_settings: DrawSettings) {
    let temp : i32;
    if draw_settings.go_fast {
        temp = (drop.length/2) - 2 + draw_settings.refresh_rate;
    }
    else {
        temp = draw_settings.refresh_rate;
    }

    if drop.frames_per_row % (temp/2+1) > 0 {
        drop.frames_per_row += 1;
    }
    else if drop.row < 1 {
        drop.frames_per_row = 1;
        drop.row += 1;
    }
    else {
        // Update the drop onscreen
        ncurses::mvaddch(drop.row - drop.length - 2, drop.col, ' ' as u32 );
        // color = primary_color;
        ncurses::attron(ncurses::COLOR_PAIR(3));
        ncurses::mvaddch(drop.row - 1, drop.col, rand::rng().random_range(MIN_CHAR..MAX_CHAR));
        ncurses::attroff(ncurses::COLOR_PAIR(3));

        ncurses::attron(ncurses::COLOR_PAIR(8));
        ncurses::mvaddch(drop.row, drop.col, rand::rng().random_range(MIN_CHAR..MAX_CHAR));
        ncurses::attroff(ncurses::COLOR_PAIR(8));

        drop.frames_per_row = 1;
        drop.row += 1;
    }

}

fn main() {
    let tick_rate = time::Duration::from_millis(20);

    let mut draw_settings: DrawSettings = DrawSettings {
        max_row: 100, max_col: 100,
        refresh_rate: 4, multicolor_trail: false,
        primary_color: 3, secondary_color: 7,
        go_fast: true
    };

    ncurses::initscr();
    let _cleanup = Cleanup;
    ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    ncurses::keypad(ncurses::stdscr(), true);
    ncurses::noecho();
    ncurses::cbreak();
    ncurses::nodelay(ncurses::stdscr(), true);

    ncurses::getmaxyx(ncurses::stdscr(), &mut draw_settings.max_row, &mut draw_settings.max_col);

    ncurses::start_color();
    ncurses::use_default_colors();
    ncurses::init_pair(1, ncurses::COLOR_BLACK, -1);
    ncurses::init_pair(2, ncurses::COLOR_RED, -1); 
    ncurses::init_pair(3, ncurses::COLOR_GREEN, -1);
    ncurses::init_pair(4, ncurses::COLOR_YELLOW, -1);
    ncurses::init_pair(5, ncurses::COLOR_BLUE, -1); 
    ncurses::init_pair(6, ncurses::COLOR_MAGENTA, -1);
    ncurses::init_pair(7, ncurses::COLOR_CYAN, -1); 
    ncurses::init_pair(8, ncurses::COLOR_WHITE, -1);

    let screen_area : u32 = (draw_settings.max_row as u32) * (draw_settings.max_col as u32);
    let n_droplets : u32 = screen_area/50;
    let mut droplets : Vec<Droplet> = Vec::new();
    
    for _ in 1..=n_droplets {
        droplets.push(Droplet::new(draw_settings.max_row, draw_settings.max_col));
    }

    loop
    {
        thread::sleep(tick_rate);
        for drop in &mut droplets {
            draw_drop(drop, draw_settings);
    
            // Check if droplet is entirely offscreen
            if drop.row > draw_settings.max_row+drop.length {
                ncurses::mvaddch(drop.row - drop.length-2, drop.col, ' ' as u32);
                *drop = Droplet::new(draw_settings.max_row, draw_settings.max_col);
            }
        }
        
        let next : i32;
        next = ncurses::getch();
        match (next as u8) as char {
            'q' => break,
            'v' => draw_settings.go_fast = !draw_settings.go_fast,
            'c' => draw_settings.multicolor_trail = !draw_settings.multicolor_trail,
            '1'..='9' => println!("User pressed {0}", next), // primary_color = next.digit()
            't' => println!("test_value = {0}", draw_settings.refresh_rate),
            _ => (),
        }

        let f1 : i32 = ncurses::KEY_F(1);
        let f8 : i32 = ncurses::KEY_F(8);
        if next >= f1 && next <= f8 {
            draw_settings.secondary_color = next - f1
        }

        match next {
            ncurses::KEY_DOWN => draw_settings.refresh_rate = std::cmp::min(draw_settings.refresh_rate+1, 20),
            ncurses::KEY_UP => draw_settings.refresh_rate = std::cmp::max(draw_settings.refresh_rate-1, 0),
            _ => (),
        }

    }

    // ncurses::endwin();
}
