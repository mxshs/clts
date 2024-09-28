use std::{
    os::unix::net::UnixStream,
    process::{
        Command,
        Stdio
    },
    io::{Read, Write},
};

fn main() {
    let mut sock = UnixStream::connect("/tmp/clts.sock")
        .expect("socket does not exist, check whether the daemon is running");

    let args: Vec<String> = std::env::args().collect();

    match sock.write_all(args[1..].join(" ").as_bytes()) {
        Ok(_) => {
            sock.shutdown(std::net::Shutdown::Write).unwrap();

            let mut cmd = String::new();

            sock.read_to_string(&mut cmd).unwrap();

            let cmd: Vec<&str> = cmd.split(" ").collect();

            let _ = Command::new(cmd[0])
                .args(cmd[1..].to_vec())
                .stdout(Stdio::inherit())
                .stdin(Stdio::inherit())
                .status();
            },
        Err(err) => {
            println!("error reading from socket: {}", err);
        }
    }
}

