//extern crate termion;

// todo: handle sigwinch https://crates.io/crates/signal-hook

use std::convert::TryFrom;
use std::io::{stdin, stdout, Stdout, Write};
use termion::color;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

use std::fs::File;
use std::io::prelude::*;

struct TextRow {
    contents: String,
}

struct ScreenBuffer {
    rows: Vec<TextRow>,
}

impl ScreenBuffer {
    fn new() -> Self {
        return Self { rows: Vec::new() };
    }
}

fn render(stdout: &mut RawTerminal<Stdout>, counter: i64, _buffer: &mut ScreenBuffer) {
    let (width, height) = termion::terminal_size().unwrap();
    write!(
        stdout,
        "{}{}{}",
        color::Bg(color::AnsiValue::grayscale(0)),
        termion::clear::All,
        termion::cursor::Goto(1, 1)
    )
    .unwrap();
    for i in 0..height {
        let mut line = format!(
            "{}{}{}{}{}",
            color::Fg(color::Black),
            color::Bg(color::White),
            i,
            color::Fg(color::Reset),
            color::Bg(color::Reset),
        );

        let extra = match i {
            _ if i == height / 3 => {
                // I want to center this
                // padding ==
                let banner = "rotide editor -- version 0.1".to_string();
                let window_center: u16 = width / 2;
                let banner_padding: u16 = u16::try_from(banner.len() / 2).unwrap();
                let padding = window_center - banner_padding;
                format!(
                    "{}{}{}\r\n",
                    termion::cursor::Left(width),
                    termion::cursor::Right(padding.into()),
                    banner,
                )
            }

            _ if i == height - 1 => format!(
                "{}{} counter: {} height: {} width: {}",
                color::Fg(color::Black),
                color::Bg(color::White),
                counter,
                height,
                width,
            ),

            _ => "\r\n".to_string(),
        };

        line.push_str(&extra);

        if i == height - 1 {
            //            turn_on_special(stdout);
            line.push_str(&format!("{}", termion::clear::AfterCursor));
            write!(stdout, "{}", line).unwrap();
            turn_off_special(stdout);
        } else {
            write!(stdout, "{}", line).unwrap();
        }
    }
    write!(stdout, "{}derp", termion::cursor::Goto(3, 1)).unwrap();
    stdout.flush().unwrap();
}

fn turn_on_special(stdout: &mut RawTerminal<Stdout>) {
    write!(
        stdout,
        "{}{}",
        color::Fg(color::Black),
        color::Bg(color::White)
    )
    .unwrap();
}

fn turn_off_special(stdout: &mut RawTerminal<Stdout>) {
    write!(
        stdout,
        "{}{}",
        color::Fg(color::Reset),
        color::Bg(color::Reset)
    )
    .unwrap();
}
fn main() {
    let buffer = ScreenBuffer::new();
    let stdin = stdin();
    let mut stdout: termion::raw::RawTerminal<std::io::Stdout> = stdout().into_raw_mode().unwrap();

    write!(
        stdout,
        "{}{}Start Typing",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
    )
    .unwrap();
    /*
    let file = File::open("/tmp/example");
    file.read_line(buffer);
     */
    let mut screen_buffer = ScreenBuffer::new();
    stdout.flush().unwrap();

    let mut counter = 0;
    for c in stdin.keys() {
        counter += 1;
        render(&mut stdout, counter, &mut screen_buffer);
        //        render();
        let a = c.unwrap();
        // handle inputs
        match a {
            Key::Ctrl('q') => break,
            _ => (), /*            Key::Backspace => write!(stdout, "{}", termion::cursor::Left(1)).unwrap(),

                                 Key::Char('\n') => write!(stdout, "\n\r").unwrap(),
                                 Key::Left => write!(stdout, "{}", termion::cursor::Left(1)).unwrap(),
                                 Key::Right => write!(stdout, "{}", termion::cursor::Right(1)).unwrap(),
                                 Key::Up => write!(stdout, "{}", termion::cursor::Up(1)).unwrap(),
                                 Key::Down => write!(stdout, "{}", termion::cursor::Down(1)).unwrap(),
                                 Key::Char(c) => write!(stdout, "{}", c).unwrap(),
                                 z => write!(stdout, "{:?}", z).unwrap(),
                     */
        };

        // render
        //

        stdout.flush().unwrap();
    }

    /*
        print!(
            "{}{}Stuff{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            termion::cursor::Goto(2, 2)
        );
        let (height, width) = termion::terminal_size().unwrap();
        print!("height{}\nwidth{}", height, width);

        io::stdout().flush().unwrap();
        thread::sleep(time::Duration::from_secs(3));
    */
}
