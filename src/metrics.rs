use crate::State;
use ansi_term::Colour;
use chrono::{DateTime, Duration, Local};

pub fn sleep_date(s: &State) -> String {
    format!(
        "Currently planning to sleep until {}",
        readable_date(DateTime::<Local>::from(
            s.user.as_ref().unwrap().date_sleep.unwrap()
        ))
    )
}
pub fn sleep_time(s: &State) -> String {
    format!(
        "You will have {} hours and {} minutes to sleep!",
        s.duration_sleep.unwrap().num_hours(),
        s.duration_sleep.unwrap().num_minutes() % 60
    )
}

pub fn sleep_cycles(s: &State) -> String {
    format!(
        "that's around {:.2} sleep cycles!",
        Colour::Yellow.bold().paint(
            ryu::Buffer::new().format(s.duration_sleep.unwrap().num_minutes() as f32 / 180.)
        )
    )
}

pub fn sleep_cycles_margin(s: &State) -> String {
    // let the user know
    // it'd be best to sleep a total of x sleep cycles tonight
    format!(
        "which is only around {:.2} sleep cycles of margin",
        Colour::Yellow.bold().paint(ryu::Buffer::new().format(
            (s.duration_sleep.unwrap().num_minutes()
                - Duration::milliseconds(s.user.as_ref().unwrap().debt as i64).num_minutes()
                - 8 * 60) as f32
                / 180.
        ))
    )
}

pub fn readable_date(date: DateTime<Local>) -> String {
    date.format("%-l:%M %p on %A, %B %eth ").to_string()
}
