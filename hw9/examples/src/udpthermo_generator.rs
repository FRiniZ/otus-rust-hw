use getopts::Options;
use msg_proto_lib::proto::smart_messages::{thermo_msg_new, udp_msg_pack};
use std::{
    env,
    net::UdpSocket,
    sync::mpsc::{channel, RecvTimeoutError},
    time::Duration,
    u32,
};

struct ThermoUDPGen {
    addr: String,
    rate: u32,
}

impl ThermoUDPGen {
    fn new(addr: String, rate: u32) -> Self {
        Self { addr, rate }
    }

    fn serv(&self) {
        let (tx, rx) = channel();

        ctrlc::set_handler(move || tx.send(()).expect("Could not send signal on channel."))
            .expect("Error setting Ctrl-C handler");

        let socket = UdpSocket::bind("127.0.0.1:8090").unwrap();
        let timeout = Duration::new(0, 1000000000 / self.rate);
        println!("Duration:{:?}", timeout);
        println!("Waiting for Ctrl-C to exit");
        let mut counter = 1;
        loop {
            let temp = 20.0 + ((counter as f32) / 2.0).sin();
            let msg = thermo_msg_new(Some(temp));
            counter += 1;
            println!("The temperature is {}", temp);

            let body = udp_msg_pack(&msg);
            let res = socket.send_to(&body, &self.addr);
            if res.is_err() {
                println!("Can't send body");
                break;
            }

            match rx.recv_timeout(timeout) {
                Ok(_) => break,
                Err(e) => {
                    if e.ne(&RecvTimeoutError::Timeout) {
                        break;
                    }
                }
            }
        }
    }
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("h", "help", "Output help");
    opts.optopt("t", "thermometer", "Address of Thermometer", "HOST:PORT");
    opts.optopt("r", "rate", "Messages per seconds(Default:2)", "INTEGER");

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

    println!("Thermometer:{server}");

    let rate = match matches.opt_str("r") {
        Some(s) => {
            let r: u32 = s.parse().unwrap();
            r
        }
        None => 2,
    };

    println!("Rate:{rate}");

    let gen = ThermoUDPGen::new(server, rate);

    gen.serv();

    println!("Bye!");
}
