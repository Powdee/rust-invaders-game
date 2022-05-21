use crossterm::event::{Event, KeyCode};
use rusty_audio::Audio;
use std::error::Error;
use std::time::Duration;

use crossterm::{terminal, event};
use std::io;

use crossterm::cursor::{Hide, Show};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::ExecutableCommand;

fn main() -> Result<(), Box<dyn Error>> {
	let mut audio = Audio::new();

	// set audio
	audio.add("explode", "explode.wav");
	audio.add("lose", "lose.wav");
	audio.add("move", "move.wav");
	audio.add("pew", "pew.wav");
	audio.add("startup", "startup.wav");
	audio.add("win", "win.wav");

	audio.play("startup");

	// terminal
	let mut stdout = io::stdout();
	terminal::enable_raw_mode()?;
	stdout.execute(EnterAlternateScreen)?;
	stdout.execute(Hide)?;

	// game loop
	'gameloop: loop {
		// input
		while event::poll(Duration::default())? {
			if let Event::Key(key_event) = event::read()? {
				match key_event.code {
					// escape button will perform audio from lose track
					KeyCode::Esc | KeyCode::Char('q') => {
						audio.play("lose");
						break 'gameloop;
					},
					// empty function 
					_ => {}
				}
			}
		}
	}

	// cleanup
	audio.wait();
	stdout.execute(Show)?;
	stdout.execute(LeaveAlternateScreen)?;

	terminal::disable_raw_mode()?;

	return Ok(());
}
