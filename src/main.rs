extern crate structopt;
mod utility;

use console::{style, Term};
use indicatif::{ProgressBar, ProgressStyle};
use rand::{Rng, rngs::ThreadRng};
use std::{thread, time};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    /// Amount of time progress bar takes to complete
    #[structopt(long = "time", short = "t")]
    time: u128,

    /// Changes progress bar to look like compiler
    #[structopt(long = "compile", short = "c")]
    compile: bool,
}

fn main() {
    let matches = Opt::from_args();

    let t = matches.time * 1000;
    let bar = ProgressBar::new(100);

    let time_per_tick = (t / 100) as u64;
    let start_time = time::Instant::now();
    let mut num_times_inc = 0;

    if let true = matches.compile {
        let rng = rand::thread_rng();

        let template_string = {
            let num_rows = Term::stdout().size().0 * 2;

            let bar_string = format!("{}{}{}{}", "{", "bar:", num_rows, "}");
            format!(
                "    {} {}{}{}",
                style("Building").bold().cyan(),
                "[",
                bar_string,
                "]"
            )
        };

        bar.set_style(
            ProgressStyle::default_bar()
                .template(template_string.as_str())
                .progress_chars("=> "),
        );

        let mut time_since_inc: u128 = 0;
        while start_time.elapsed().as_millis() < t || num_times_inc < 100 {
            let start_loop_time = time::Instant::now();

            thread::sleep(time::Duration::from_millis(50));

            if time_since_inc >= time_per_tick as u128 {
                bar.println(compile_message(rng));
                bar.inc(1);
                time_since_inc = time_since_inc - time_per_tick as u128;
                num_times_inc += 1;
            }
            time_since_inc += start_loop_time.elapsed().as_millis();
        }

        bar.finish_and_clear();
    } else {
        while start_time.elapsed().as_millis() < t {
            bar.inc(1);
            thread::sleep(time::Duration::from_millis(time_per_tick));
        }
    }

    println!("Complete");
}

fn compile_message(mut rng: ThreadRng) -> String {
    let message = format!(
        "  {}: {}_{} v{}.{}.{}",
        style("Compiling").bold().green(),
        utility::ADJECTIVES[(rng.gen_range(0, utility::ADJECTIVES_LENGTH)) as usize],
        utility::NOUNS[(rng.gen_range(0, utility::NOUNS_LENGTH)) as usize],
        rng.gen_range(0, 15),
        rng.gen_range(0, 15),
        rng.gen_range(1, 15),
    );

    message
}
