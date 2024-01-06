use getopts::Options;
use msg_proto_lib::netmessage::ErrorNetMessage;
use msg_proto_lib::{
    netmessage::NetMessage,
    proto::cmd_messages::{cmd_request_new, cmd_request_pack, cmd_responce_unpack},
};
use std::env;
use tokio::net::TcpStream;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn print_menu() {
    println!("1. Power");
    println!("2. On");
    println!("3. Off");
    println!("4. Exit");
}

fn read_command() -> String {
    let mut cmd: String = String::new();
    loop {
        print_menu();
        std::io::stdin()
            .read_line(&mut cmd)
            .expect("Error read cmd");
        cmd.pop(); //remove new line
        match cmd.as_str() {
            "1" => return String::from("power"),
            "2" => return String::from("on"),
            "3" => return String::from("off"),
            "4" => return String::from("exit"),
            _ => println!("Error cmd. Try again"),
        }
    }
}

#[tokio::main]
async fn main() {
    let args: Vec<_> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("h", "help", "Output help");
    opts.optopt("c", "Connect to SmartSocket", "", "HOST:PORT");

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

    let server = match matches.opt_str("c") {
        Some(s) => s,
        _ => {
            print_usage(&program, opts);
            return;
        }
    };

    let mut _stream = TcpStream::connect(server).await.unwrap();

    println!(
        "Connecting to SmartSocketServer:{}",
        _stream.peer_addr().unwrap()
    );
    let mut buff: Vec<u8> = Vec::with_capacity(1024);
    loop {
        let cmd: String = read_command();
        if cmd.as_str() == "exit" {
            break;
        }
        let data = cmd_request_pack(&cmd_request_new(cmd, vec![]));
        let _ = NetMessage::send(&mut _stream, data).await;
        loop {
            let ret = NetMessage::recv(&mut _stream, &mut buff).await;
            match ret {
                Ok(msg) => {
                    println!("{}", msg);
                    let repl = cmd_responce_unpack(msg.body_as_bytes()).unwrap();
                    println!("{}", repl);
                    break;
                }
                Err(e) => match e {
                    ErrorNetMessage::IOError(msg) => panic!("{}", msg),
                    ErrorNetMessage::Blocked(_) => continue,
                    ErrorNetMessage::NotReady => continue,
                },
            }
        }
    }

    println!("Bye!");
}
