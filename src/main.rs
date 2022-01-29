use chrono::{prelude::*, Duration};
use do_you_really_want_to_procrastinate::changetime::change_date_sleep;
use do_you_really_want_to_procrastinate::metrics::sleep_cycles;
use do_you_really_want_to_procrastinate::metrics::sleep_date;
use do_you_really_want_to_procrastinate::metrics::sleep_time;
use do_you_really_want_to_procrastinate::State;
use do_you_really_want_to_procrastinate::User;
use std::error::Error;
use std::fs;
use std::io;
use std::ops::Add;

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

fn main() -> Result<()> {
    let mut s = State {
        data_path: None,
        now: None,
        tomorrow: None,
        duration_sleep: None,
        user: None,
    };

    s.user = Some(get_user(&mut s)?);

    s.now = Some(Local::now());

    // check if we should commit the debt pending situation
    if let Some(date_sleep) = s.user.as_ref().unwrap().date_sleep {
        if s.now.unwrap() > date_sleep {
            if let Some(sleep_duration) = s.user.as_ref().unwrap().sleep_duration {
                // saturating sub seems useful!
                s.user.as_mut().unwrap().debt =
                    s.user.as_ref().unwrap().debt.saturating_sub(sleep_duration);
                s.user.as_mut().unwrap().sleep_duration = None;
            }
        }
    }

    // set time to hour 6 and minute 45, second 0
    s.tomorrow = if let Some(sleep_date) = s.user.as_ref().unwrap().date_sleep {
        Some(sleep_date.into())
    } else {
        Some({
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
        })
    };

    s.duration_sleep = Some(s.tomorrow.unwrap().signed_duration_since(s.now.unwrap()));

    if s.user.as_ref().unwrap().date_sleep.is_some() {
        println!("{}", sleep_date(&s));
    }
    println!("{}", sleep_time(&s));
    println!("{}", sleep_cycles(&s));
    // if there is some debt
    // seems like useful information to have regardless of if there is debt or not??
    // if user.debt > 0 {
    // }
    let mut counter = 0;
    let mut input = String::new();
    let mut update = true;
    loop {
        input.clear();
        println!("sleep? (y/n)");
        io::stdin().read_line(&mut input)?;
        match input.trim().to_uppercase().as_str() {
            "Y" => {
                println!("Prepare your bag.");
                println!("Fill up the water bottle.");
                println!("If you're not taking a shower, just take off your contacts, then head straight to bed.");
                println!("Don't worry. Tomorrow will be a brighter day.");
                s.user.as_mut().unwrap().date_sleep = Some(s.tomorrow.unwrap().into());
                s.user.as_mut().unwrap().sleep_duration =
                    Some(s.duration_sleep.unwrap().num_milliseconds() as u64);
                break;
            }
            "N" => {
                println!("Ok then.");
                println!("Make sure you're not procrastinating! Don't forget, sleep is one of the most important things you can get.");
                // cancel date sleep
                s.user.as_mut().unwrap().date_sleep = None;
                s.user.as_mut().unwrap().sleep_duration = None;
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
                        s.user.as_mut().unwrap().debt =
                            Duration::minutes(((8. - v) * 60.).floor() as i64).num_milliseconds()
                                as u64;
                        break;
                    }
                }
                break;
            }
            "Q" => {
                update = false;
                break;
            }
            "CUSTOM" => change_date_sleep(&mut s)?,
            // "CHECK" => {
            //     println!("", DateTime::<Local>::from(s.user.))
            // },
            _ => {
                counter += 1;
                if counter % 3 == 0 {
                    println!("remember, don't temporize!");
                    println!("be decisive - Faker");
                } else {
                    println!("..");
                }
            }
        }
    }
    if update {
        // write and save it to json
        fs::write(
            &s.data_path.unwrap(),
            serde_json::to_string_pretty(s.user.as_ref().unwrap()).unwrap(),
        )?;
    }
    Ok(())
}
