extern crate termion;
use termion::{clear, color, style, cursor};
use termion::raw::IntoRawMode;
use termion::input::TermRead;
use termion::event::Key;
use std::io::{Write, stdin, stdout};

fn main() {
    let mut stdout = stdout().into_raw_mode().unwrap();
    let menu_options = vec!("1. Greet", "2. Leave rudely");

    clear(& mut stdout);

    let mut selected_index = 0;
    render(& mut stdout, selected_index,  & menu_options);

    stdout.flush().unwrap();

    for c in stdin().keys() {
        match c.unwrap() {
            // Exit.
            Key::Char('q') => break,
            Key::Char('\n') => {
                if selected_index == 0 {
                    println!("Hello!\n\r");
                    break;
                } else {
                    println!("So long sucker!!!\n\r");
                    break;
                }
            },
            Key::Char(c)   => println!("{}", c),
            Key::Alt(c)    => println!("Alt-{}", c),
            Key::Ctrl(c)   => println!("Ctrl-{}", c),
            Key::Left      => println!("<left>"),
            Key::Right     => println!("<right>"),
            Key::Up        => {
                if selected_index == 0 { 
                    selected_index = menu_options.len() - 1 
                } else {
                    selected_index -= 1;
                }
            },
            Key::Down      => {
                if selected_index == menu_options.len() - 1 { 
                    selected_index = 0;
                } else {
                    selected_index += 1;
                }
            },
            _              => println!("Other"),
        }

        clear(& mut stdout);
        render(& mut stdout, selected_index,  & menu_options);
        stdout.flush().unwrap();
    }

    // Show the cursor again before we exit.
    write!(stdout, "{}", termion::cursor::Show).unwrap();
}

fn clear(stdout: & mut termion::raw::RawTerminal<std::io::Stdout>) {
    // Clear the screen
    write!(stdout, "{clear}{goto}{cursor_off}", 
             clear = clear::All,
             goto = cursor::Goto(1, 1),
             cursor_off = cursor::Hide,
            ).unwrap();
}

fn render(stdout: & mut termion::raw::RawTerminal<std::io::Stdout>, selected_index: usize, menu_options: & Vec<&str>) {
    for (index, menu_option) in menu_options.iter().enumerate() {
        let style = if selected_index == index { format!("{}", style::Invert) } else { format!("{}", style::NoInvert) };
        write!(stdout, "{style}{option}{invert_off}\n\r", 
               style = style,
               option = menu_option,
               invert_off = style::NoInvert);
    }
}
