use ansi_term::Colour;
use chrono::{prelude::*, Duration};
use do_you_really_want_to_procrastinate::User;
use std::error::Error;
use std::fs;
use std::io;
use std::ops::Add;

fn main() -> Result<(), Box<dyn Error>> {
    let now: DateTime<Local> = Local::now();
    // set time to hour 6 and minute 45, second 0
    let tomorrow = now
        .with_hour(6)
        .unwrap()
        .with_minute(45)
        .unwrap()
        .with_second(0)
        .unwrap();

    let mut user = if let Ok(f) = fs::read_to_string("data.json") {
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

    // check if we should commit the debt pending situation
    if let Some(date_sleep) = user.date_sleep {
        if now > date_sleep {
            if let Some(debt_pending) = user.sleep_duration {
                // saturating sub seems useful!
                user.debt = user.debt.saturating_sub(debt_pending);
                user.sleep_duration = None;
            }
        }
    }

    // if we're less than that time, it means we stayed up
    let duration_sleep = if now < tomorrow {
        println!("damn, stayed up huh?");
        tomorrow.signed_duration_since(now)
    } else {
        // otherwise, we can proceed as normal and add a day to our time
        tomorrow.add(Duration::days(1)).signed_duration_since(now)
    };
    println!(
        "You will have {} hours and {} minutes to sleep!",
        duration_sleep.num_hours(),
        duration_sleep.num_minutes() % 60
    );
    println!(
        "that's around {:.2} sleep cycles!",
        Colour::Yellow
            .bold()
            .paint(&format!("{}", duration_sleep.num_minutes() as f32 / 180.))
    );
    // if there is some debt
    if user.debt > 0 {
        // let the user know
        // it'd be best to sleep a total of x sleep cycles tonight
        println!(
            "that's only around {:.2} sleep cycles of margin",
            Colour::Yellow.bold().paint(&format!(
                "{}",
                (duration_sleep.num_minutes()
                    - Duration::milliseconds(user.debt as i64).num_minutes()
                    - 8 * 60) as f32
                    / 180.
            ))
        );
    }
    let mut counter = 0;
    println!("sleep? (y/n)");
    let mut input = String::new();
    let mut updated = false;
    loop {
        io::stdin().read_line(&mut input)?;
        match input.trim().to_uppercase().as_str() {
            "Y" => {
                println!("Prepare your bag.");
                println!("Fill up the water bottle.");
                println!("If you're not taking a shower, just take off your contacts, then head straight to bed.");
                println!("Don't worry. Tomorrow will be a brighter day.");
                user.date_sleep = Some(tomorrow.into());
                user.sleep_duration = Some(duration_sleep.num_milliseconds() as u64);
                updated = true;
                break;
            }
            "N" => {
                println!("Ok then.");
                println!("Make sure you're not procrastinating! Don't forget, sleep is one of the most important things you can get.");
                // cancel date sleep
                user.date_sleep = None;
                user.sleep_duration = None;
                updated = true;
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
                updated = true;
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
    if updated {
        // write and save it to json
        fs::write("data.json", serde_json::to_string_pretty(&user)?)?
    }
    Ok(())
}
