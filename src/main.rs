use ansi_term::Colour;
use chrono::{prelude::*, Duration};
use do_you_really_want_to_procrastinate::User;
use std::error::Error;
use std::fs;
use std::io;
use std::ops::Add;
use std::path::PathBuf;
type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn get_user(state: &mut State) -> Result<User> {
    state.data_path = Some([env!("CARGO_MANIFEST_DIR"), "data.json"].iter().collect());

    let user = if let Ok(f) = fs::read_to_string(state.data_path.as_ref().unwrap()) {
        if let Ok(u) = serde_json::from_str::<User>(&f) {
            u
        } else {
            eprintln!("corrupt data");
            std::process::exit(1);
        }
    } else {
        User {
            debt: 0,
            sleep_duration: None,
            date_sleep: None,
        }
    };

    Ok(user)
}

struct State {
    data_path: Option<PathBuf>,
    now: Option<DateTime<Local>>,
    tomorrow: Option<DateTime<Local>>,
    duration_sleep: Option<Duration>,
}

fn main() -> Result<()> {
    let mut s = State {
        data_path: None,
        now: None,
        tomorrow: None,
        duration_sleep: None,
    };

    let mut user = get_user(&mut s)?;

    s.now = Some(Local::now());

    // check if we should commit the debt pending situation
    if let Some(date_sleep) = user.date_sleep {
        if s.now.unwrap() > date_sleep {
            if let Some(sleep_duration) = user.sleep_duration {
                // saturating sub seems useful!
                user.debt = user.debt.saturating_sub(sleep_duration);
                user.sleep_duration = None;
            }
        }
    }

    // set time to hour 6 and minute 45, second 0
    s.tomorrow = Some({
        let tentative_tomorrow = s
            .now
            .unwrap()
            .with_hour(6)
            .unwrap()
            .with_minute(45)
            .unwrap()
            .with_second(0)
            .unwrap();

        // if we're less than that time, it means we stayed up
        if s.now.unwrap() < tentative_tomorrow {
            println!("damn, stayed up huh?");
            tentative_tomorrow
        } else {
            tentative_tomorrow.add(Duration::days(1))
        }
    });

    s.duration_sleep = Some(s.tomorrow.unwrap().signed_duration_since(s.now.unwrap()));

    println!(
        "You will have {} hours and {} minutes to sleep!",
        s.duration_sleep.unwrap().num_hours(),
        s.duration_sleep.unwrap().num_minutes() % 60
    );
    println!(
        "that's around {:.2} sleep cycles!",
        Colour::Yellow.bold().paint(
            ryu::Buffer::new().format(s.duration_sleep.unwrap().num_minutes() as f32 / 180.)
        )
    );
    // if there is some debt
    if user.debt > 0 {
        // let the user know
        // it'd be best to sleep a total of x sleep cycles tonight
        println!(
            "that's only around {:.2} sleep cycles of margin",
            Colour::Yellow.bold().paint(ryu::Buffer::new().format(
                (s.duration_sleep.unwrap().num_minutes()
                    - Duration::milliseconds(user.debt as i64).num_minutes()
                    - 8 * 60) as f32
                    / 180.
            ))
        );
    }
    let mut counter = 0;
    println!("sleep? (y/n)");
    let mut input = String::new();
    loop {
        io::stdin().read_line(&mut input)?;
        match input.trim().to_uppercase().as_str() {
            "Y" => {
                println!("Prepare your bag.");
                println!("Fill up the water bottle.");
                println!("If you're not taking a shower, just take off your contacts, then head straight to bed.");
                println!("Don't worry. Tomorrow will be a brighter day.");
                user.date_sleep = Some(s.tomorrow.unwrap().into());
                user.sleep_duration = Some(s.duration_sleep.unwrap().num_milliseconds() as u64);
                break;
            }
            "N" => {
                println!("Ok then.");
                println!("Make sure you're not procrastinating! Don't forget, sleep is one of the most important things you can get.");
                // cancel date sleep
                user.date_sleep = None;
                user.sleep_duration = None;
                break;
            }
            "WAIT" => {
                println!(
                    "Ok, let's readjust your debt. How many hours of sleep did you get last night?"
                );
                loop {
                    input.clear();
                    io::stdin().read_line(&mut input)?;
                    if let Ok(v) = input.trim().parse::<f32>() {
                        // this isn't very readable
                        user.debt = Duration::minutes(((8. - v) * 60.).floor() as i64)
                            .num_milliseconds() as u64;
                        break;
                    }
                }
                break;
            }
            _ => {
                counter += 1;
                if counter % 3 == 0 {
                    println!("remember, don't temporize!");
                    println!("be decisive - Faker");
                } else {
                    println!("..");
                }
                input.clear();
            }
        }
    }
    // write and save it to json
    fs::write(&s.data_path.unwrap(), serde_json::to_string_pretty(&user)?)?;
    Ok(())
}
