use cfonts::{render, Fonts, Options};
use std::io;
use std::time::Instant;

use time::Duration;
use tui::{
    backend::CrosstermBackend,
    //text::Span,
    //widgets::{Block, Borders, Paragraph},
    widgets::Paragraph,
    Terminal,
};

use crossterm::{
    event::{self, DisableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

pub const WORK_TIME: Duration = Duration::new(1499, 0);
pub const RELAX_TIME: Duration = Duration::new(899, 0);

enum ClockState {
    Work,
    Relax,
}

impl std::fmt::Display for ClockState{
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!("
        match self{
            ClockState::Work => write!(f, "Time to Work"),
            ClockState::Relax => write!(f, "Time to Relax"),
        }
    }
}

struct Clock {
    time: Instant,
    pub state: ClockState,
}

impl Clock {
    pub fn start() -> Self {
        Clock {
            time: Instant::now(),
            state: ClockState::Work,
        }
    }
    pub fn left(&self) -> Duration {
        match self.state {
            ClockState::Work => WORK_TIME - self.time.elapsed(),
            ClockState::Relax => RELAX_TIME - self.time.elapsed(),
        }
    }
    pub fn reset(&mut self) {
        match self.state {
            ClockState::Work => {
                self.time = Instant::now();
                self.state = ClockState::Relax;
            }
            _ => {
                self.time = Instant::now();
                self.state = ClockState::Work;
            }
        }
    }
}

fn main() -> Result<(), io::Error> {
    let mut sout = io::stdout();
    enable_raw_mode()?;
    execute!(sout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(sout);
    let mut term = Terminal::new(backend)?;
    let mut clock = Clock::start();

    let mut quit = false;

    while quit == false {
        if clock.left().cmp(&Duration::ZERO).is_le() {
            clock.reset();
        }

        let secs_raw = clock.left().as_seconds_f32();
        let min = secs_raw as i32 / 60;
        let secs = secs_raw as i32 - min * 60;
        let output = render(Options {
            text: format!("{}:{}", min, secs),
            font: Fonts::FontBlock,
            ..Options::default()
        });
        

        term.draw(|f| {
            let p = Paragraph::new(format!("{}\n{}",clock.state,output.text)).alignment(tui::layout::Alignment::Center);
            
            f.render_widget(p, f.size());
        })?;
        term.autoresize()?;

        if event::poll(std::time::Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => {
                        quit = true;
                    }
                    KeyCode::Char(' ') => {
                        clock.reset();
                    }
                    _ => (),
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(
        term.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    term.show_cursor()?;

    Ok(())
}
