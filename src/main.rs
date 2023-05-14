use anyhow::{Context, Result};
/// cargo run -- clap Cargo.toml
use clap::Parser;
#[macro_use]
extern crate log;

/// Search for parttern in a file and displays the lines that contains it.
#[derive(Debug, Clone, Parser)]
struct Cli {
    /// The pattern to look for 
    pattern: String,
    /// The path of the file to read
    path: std::path::PathBuf,
}

/// state of the app
#[derive(Debug)]
struct Counter {
    // The counter value
    value: i32,
}

/// User interactions 
#[derive(Debug, Clone, Copy)]
pub enum Message {
    IncrementPressed,
    DecrementPressed,
}

use iced::widget::{button, column, text, Column};

impl Counter {
    pub fn view(&self) -> Column<Message> {
        // We use a column: a simple vertical layout
        column![
            // The increment button. We tell it to produce an
            // `IncrementPressed` message when pressed
            button("+").on_press(Message::IncrementPressed),

            // We show the value of the counter here
            text(self.value).size(50),

            // The decrement button. We tell it to produce a
            // `DecrementPressed` message when pressed
            button("-").on_press(Message::DecrementPressed),
        ]
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::IncrementPressed => {
                self.value += 1;
            }
            Message::DecrementPressed => {
                self.value -= 1;
            }
        }
    }
}

fn main() -> Result<()> {
    env_logger::init(); // loging 
    info!("starting up");
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;

    // println!("pattern: {pattern:?}\npath: {path:?}");

    //println!("cli {cli:?}");
    let mut i:i32 = 0;
    for line in content.lines() {
        i += 1;
        if line.contains(&args.pattern) {
            println!("{} : {}", i, line);
        }
    }

    info!("shuting down");
    Ok(())
}
