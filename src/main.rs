extern crate termion;

use termion::clear;
use termion::event::*;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::cursor;
use termion::terminal_size;
use termion::style;
use std::io::{Write, stdout, stdin};

struct Window<R, W> {
    width: u16,
    height: u16,
//    panes: Vec<Pane>,
    stdout: W,
    stdin: R,
    lines: Vec<Line>,
    cursor_pos_x: u16,
    cursor_pos_y: u16,
}

impl <R, W: Write> Window <R, W> {
    fn draw(&mut self) {
       write!(self.stdout, "{}", clear::All);
       write!(self.stdout, "{}", cursor::Goto(1,1));
       let mut cursor_pos_y: u16 = 1;
       for x in &self.lines {
         write!(self.stdout, "{}", cursor::Goto(1,cursor_pos_y));
         write!(self.stdout, "{}", x.text, );
         cursor_pos_y += 1;
       }

       let size = terminal_size().unwrap();
       write!(self.stdout, "{}x{}", size.0, size.1);
 
       self.stdout.flush().unwrap();
    }
    fn quit(&mut self) {
       write!(self.stdout, "{}{}{}", clear::All, style::Reset, cursor::Goto(1, 1)).unwrap();
    }
}

struct Pane {
    x: u16,
    y: u16,
    width: u16, //0-100 percentage?
    height: u16
}

struct Line {
    text: String
}

fn main() {
    let stdin = stdin();
    let stdin = stdin.lock();
    let stdout = stdout().into_raw_mode().unwrap();
    let stdout = stdout.lock();
    let mut window = Window {
        width: 0,
        height: 0,
        stdin: stdin.events(),
        stdout: stdout,
        lines: vec![Line { text: "Hey1".to_owned()},
                    Line { text: "Hey2".to_owned()},
                    Line { text: "Hey3".to_owned()},
                    Line { text: "This is a really long string that should go off the side somehow what will happen?".to_owned()}
                         ],
        cursor_pos_x: 0,
        cursor_pos_y: 0,

    };

    loop {
        let evt = window.stdin.next().unwrap().unwrap();
        match evt {
            Event::Key(Key::Char('q')) => break,
            Event::Key(Key::Char('r')) => {
                window.draw();
            }
            Event::Mouse(_) => {},
            _ => {}
        }
    }

    window.quit();
}