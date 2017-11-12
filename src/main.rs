extern crate termion;

use termion::clear;
use termion::event::*;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::cursor;
use termion::terminal_size;
use termion::{color, style};
use std::io::{Write, stdout, stdin};

struct Window<R, W> {
    width: u16,
    height: u16,
//    panes: Vec<Pane>,
    stdout: W,
    stdin: R,
    cursor_x: u16,
    cursor_y: u16,
}

struct Buffer {
    width: u16,
    height: u16,
    x: u16,
    y: u16,
    content: Vec<PrettyString>
}

struct PrettyString {
    text: String,
    fg: color::Rgb,
    bg: color::Rgb,
}

impl PrettyString {
    fn new(text:String, fg: color::Rgb, bg: color::Rgb) -> PrettyString {
        PrettyString {
            text,
            fg,
            bg,
        }
    }
    fn draw_pretty(&self) -> String {
        format!("{}{}{}{}",
                color::Fg(self.fg),
                color::Bg(self.bg),
                self.text,
                style::Reset)
    }
}

enum Command {
    Clear,
    Goto(u16,u16),
    Write(String),
}



impl <R, W: Write> Window <R, W> {
    fn draw(&mut self) {

       use Command::*;
       self.draw_command(Clear);
       self.draw_command(Goto(1,1));
       let size = terminal_size().unwrap();
       write!(self.stdout, "{}x{}", size.0, size.1);
       self.flush();

    }

    fn draw_buffer(&mut self, buffer: &Buffer) {
        use Command::*;
        let mut y = buffer.y;
        //line is a PrettyString
        for line in &buffer.content {
            self.draw_command(Goto(buffer.x, y));
            if line.text.len() as u16 <= buffer.width {
                self.draw_command(Write(line.draw_pretty().clone()));
            } else {
                let dotdotdot = String::from("...");
                let shorter_string = line.text.get(0..17).unwrap().to_owned();
                let short_plus_dots = shorter_string + &dotdotdot;
                let short_pretty = PrettyString::new(short_plus_dots,line.fg,line.bg);
                self.draw_command(Write(short_pretty.draw_pretty().clone()));
            }
            y += 1;

        }
       self.flush();
    }

    fn draw_command(&mut self, command: Command) {
        match command {
            Command::Clear => write!(self.stdout, "{}", clear::All).unwrap(),
            Command::Goto(x,y) => write!(self.stdout, "{}", cursor::Goto(x,y)).unwrap(),
            Command::Write(thing) => write!(self.stdout, "{}", thing).unwrap(),
        };
    }

    fn flush(&mut self) {
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

#[derive(Clone)]
struct Line {
    text: String
}

fn main() {
    let stdin = stdin();
    let stdin = stdin.lock();
    let stdout = stdout().into_raw_mode().unwrap();
    let stdout = stdout.lock();

    let white = color::Rgb(255,255,255);
    let dark_grey = color::Rgb(50,50,50);
    let black = color::Rgb(0,0,0);

    let mut p_strings = Vec::new();
    p_strings.push(PrettyString::new("P Line 1".to_owned(), white, black));
    p_strings.push(PrettyString::new("P Line 2 this line is really long so it should get clipped off".to_owned(), white, dark_grey));
    p_strings.push(PrettyString::new("P Line 3 10 14 17 20 more".to_owned(), white, black));
    p_strings.push(PrettyString::new("This one should be20".to_owned(), white, black));
    p_strings.push(PrettyString::new("P Line 4".to_owned(), white, black));

    let mut buffer = Buffer {
        width: 20,
        height: 20,
        x: 2,
        y: 2,
        content: p_strings
    };
    let mut window = Window {
        width: 0,
        height: 0,
        stdin: stdin.events(),
        stdout: stdout,
        cursor_x: 1,
        cursor_y: 1,

    };

    loop {
        let evt = window.stdin.next().unwrap().unwrap();
        match evt {
            Event::Key(Key::Char('q')) => break,
            Event::Key(Key::Char('r')) => {
                window.draw();
                window.draw_buffer(&buffer);
                
            }
            Event::Mouse(_) => {},
            _ => {}
        }
    }

    window.quit();
}