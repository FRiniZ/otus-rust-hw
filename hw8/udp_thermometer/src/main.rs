use getopts::Options;
use std::{
    env,
    net::UdpSocket,
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc::{sync_channel, RecvTimeoutError, SyncSender},
        Arc, Mutex,
    },
    thread,
    time::Duration,
};

use udp_msg_lib::proto::thermo_msg_unpack;

struct ThermometerUDP {
    temperature: Arc<Mutex<f32>>,
    finished: Arc<AtomicBool>,
}

impl ThermometerUDP {
    fn new(addr: String, tx: Arc<SyncSender<()>>) -> Self {
        let socket = UdpSocket::bind(addr).unwrap();

        let temperature = Arc::new(Mutex::new(0.0));
        let finished = Arc::new(AtomicBool::new(false));

        let _temperate_clone = Arc::clone(&temperature);
        let finished_clone = Arc::clone(&finished);

        thread::spawn(move || loop {
            if finished_clone.load(Ordering::SeqCst) {
                return;
            }
            let mut buf = [0; 1024];
            let res = socket.recv_from(&mut buf);
            if res.is_err() {
                println!("Cant read data{}", res.unwrap_err());
                break;
            }

            let (len, _) = res.unwrap();

            let res = thermo_msg_unpack(&buf[0..len]);
            if res.is_err() {
                println!("Error:{}", res.unwrap_err());
                tx.send(()).expect("Cant sent message to channel");
                break;
            }
            let msg = res.unwrap();
            {
                *_temperate_clone.lock().unwrap() = msg.temperature.unwrap();
            }
        });
        Self {
            temperature,
            finished,
        }
    }
}

impl Drop for ThermometerUDP {
    fn drop(&mut self) {
        println!("Drop ThermometerUDP");
        self.finished.store(true, Ordering::SeqCst)
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

    let (tx, rx) = sync_channel(1);
    let sender = Arc::new(tx);
    let thermo = ThermometerUDP::new(server, sender.clone());

    ctrlc::set_handler(move || sender.send(()).expect("Could not send signal on channel."))
        .expect("Error setting Ctrl-C handler");

    let timeout = Duration::new(0, 1000000000);
    println!("Waiting for Ctrl-C to exit");

    loop {
        {
            let tempo = thermo.temperature.lock().unwrap();
            println!("The temperature is {tempo}");
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

    println!("Bye!");
}
