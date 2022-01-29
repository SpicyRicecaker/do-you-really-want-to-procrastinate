use std::io::{self, Write};

use chrono::Duration;
use crossterm::event::KeyModifiers;
pub use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute, queue, style,
    terminal::{self, ClearType},
    Command, Result,
};

use crate::{
    metrics::{readable_date, sleep_cycles, sleep_cycles_margin, sleep_time},
    State,
};

pub fn change_date_sleep(s: &mut State) -> Result<()> {
    let default_sleep_date = s.tomorrow.unwrap();
    // TODO not sure if we need to clone
    // let mut tentative_sleep_date = default_sleep_date;

    let mut stdout = io::stdout();
    execute!(&mut stdout, terminal::EnterAlternateScreen)?;

    terminal::enable_raw_mode()?;

    let mut full_quit = false;
    loop {
        // clear screen
        queue!(
            &mut stdout,
            style::ResetColor,
            terminal::Clear(ClearType::All),
            cursor::Hide,
            cursor::MoveTo(1, 1)
        )?;

        let prompt = format!(
            r#"Wow, adjusting the sleep time, huh? It must be the weekend!
Setting new sleep date from (old){} to (new){}.
{}
{}
{}
Type 'y' when you're done! (or 'n' to cancel)"#,
            readable_date(default_sleep_date),
            readable_date(s.tomorrow.unwrap()),
            sleep_time(s),
            sleep_cycles(s),
            sleep_cycles_margin(s)
        );

        // Manually writes the MENU static string to stdout
        prompt
            .split('\n')
            .try_for_each(|line| queue!(stdout, style::Print(line), cursor::MoveToNextLine(1)))
            .unwrap();

        stdout.flush()?;
        if let Event::Key(KeyEvent { code, modifiers }) = event::read()? {
            match code {
                // TODO should probably move it into a struct so we don't have to destructure
                // We're storing and mutating a f32 two times, one in the audio thread and one in main thread.
                // shift + j,k increments minute
                KeyCode::Char('J') => {
                    s.tomorrow = Some(s.tomorrow.unwrap() - Duration::minutes(1));
                    s.duration_sleep =
                        Some(s.tomorrow.unwrap().signed_duration_since(s.now.unwrap()));
                }
                KeyCode::Char('j') => {
                    // decrement minute
                    // if modifiers == KeyModifiers::SHIFT {
                    // probably shouldn't be writing to user directly but
                    // not doing so would require more abstractions that I'm
                    // not sure will be useful in the long run
                    // } else {
                    // decrement hour
                    s.tomorrow = Some(s.tomorrow.unwrap() - Duration::hours(1));
                    // }
                    s.duration_sleep =
                        Some(s.tomorrow.unwrap().signed_duration_since(s.now.unwrap()));
                }
                KeyCode::Char('K') => {
                    s.tomorrow = Some(s.tomorrow.unwrap() + Duration::minutes(1));
                    s.duration_sleep =
                        Some(s.tomorrow.unwrap().signed_duration_since(s.now.unwrap()));
                }
                KeyCode::Char('k') => {
                    // if modifiers == KeyModifiers::SHIFT {
                    //     s.tomorrow = Some(s.tomorrow.unwrap() + Duration::minutes(1));
                    // } else {
                    // decrement hour
                    s.tomorrow = Some(s.tomorrow.unwrap() + Duration::hours(1));
                    // }
                    s.duration_sleep =
                        Some(s.tomorrow.unwrap().signed_duration_since(s.now.unwrap()));
                }
                KeyCode::Char('y') => {
                    break;
                }
                KeyCode::Char('q') => {
                    full_quit = true;
                    break;
                }
                KeyCode::Char('c') => {
                    if modifiers == KeyModifiers::CONTROL {
                        full_quit = true;
                        break;
                    }
                }
                KeyCode::Char('n') => {
                    // set sleep date back to default time
                    s.tomorrow = Some(default_sleep_date);
                    break;
                }
                // j,k increments hour
                // 'y' accepts current values and immediately writes into sleep
                _ => {}
            }
        };
    }

    // Release cursor
    execute!(
        &mut stdout,
        style::ResetColor,
        cursor::Show,
        terminal::LeaveAlternateScreen
    )?;

    terminal::disable_raw_mode()?;

    if full_quit {
        std::process::exit(0);
    }

    Ok(())
}
