use getopts::Options;
use hw7::smartnet::message::Message;
use std::{env, net::TcpStream};

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("h", "help", "Output help");
    opts.reqopt("s", "server", "Address of server", "HOST:PORT");
    opts.optopt("r", "room", "Name of room", "NAME");
    opts.optopt("d", "device", "Name of device", "NAME");
    opts.optopt("", "cmd", "Command for device", "CMD");
    opts.optflag("", "report", "Output report of house");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            println!("{}", f);
            print_usage(&program, opts);
            return;
        }
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    let server = match matches.opt_str("s") {
        Some(s) => s,
        _ => {
            print_usage(&program, opts);
            return;
        }
    };

    println!("Server:{}", server);

    let room = matches.opt_str("r");
    if room.is_some() {
        println!("Room:{}", room.as_ref().unwrap());
    }

    let device = matches.opt_str("d");
    if device.is_some() {
        println!("Device:{}", device.as_ref().unwrap());
    }

    let cmd = matches.opt_str("cmd");
    if cmd.is_some() {
        println!("Cmd:{}", cmd.as_ref().unwrap());
    }

    let cmd_report = matches.opt_present("report");
    println!("Cmd report:{}", cmd_report);

    let report = if cmd_report {
        Some("Request report".to_string())
    } else {
        None
    };

    match TcpStream::connect(server.clone()) {
        Ok(mut stream) => {
            println!("Successfully connected to server[{}]", server.clone());
            let msg = Message {
                report,
                room,
                device,
                cmd,
            };
            let ret = Message::send(&mut stream, msg);
            if ret.is_err() {
                panic!("Can't send message{:?}", ret);
            }
            let repl = Message::recv(&mut stream);
            match repl {
                Ok(msg) => {
                    handle_reply(msg);
                }
                Err(e) => {
                    eprintln!("{}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to connect: {}", e);
        }
    }
}

fn handle_reply(msg: Message) {
    if msg.report.is_some() {
        println!("Report received:\n{}", msg.report.unwrap());
        return;
    }

    if msg.cmd.is_some() {
        println!("Result command:{}", msg.cmd.unwrap());
    }
}
