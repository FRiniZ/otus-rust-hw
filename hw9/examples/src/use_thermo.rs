use getopts::Options;
use std::{
    env,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

use tokio::time::Duration;
use tokio::{self, time::interval};
use udp_thermometer::SmartThermo;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

#[tokio::main]
async fn main() {
    let args: Vec<_> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("h", "help", "Output help");
    opts.optopt(
        "t",
        "thermometer",
        "Address of External UDPThermometer",
        "HOST:PORT",
    );

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => {
            println!("{}", e);
            print_usage(&program, opts);
            return;
        }
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    let server = match matches.opt_str("t") {
        Some(s) => s,
        _ => {
            print_usage(&program, opts);
            return;
        }
    };

    println!("UDP Generator:{server}");

    let finished = Arc::new(AtomicBool::new(false));
    let finished_clone = Arc::clone(&finished);
    let thermo = SmartThermo::new(server, finished.clone()).await;

    ctrlc_async::set_handler(move || {
        finished_clone.store(true, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    println!("Waiting for Ctrl-C to exit");

    let mut interval = interval(Duration::from_millis(1000));

    while !finished.load(Ordering::SeqCst) {
        interval.tick().await;
        let tempo = thermo.temperature().await;
        println!("The temperature is {tempo}");
    }

    println!("Bye!");
}
