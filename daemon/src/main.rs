use std::{
    path,
    os::unix::net::UnixListener,
    io::{
        Read,
        Write,
    },
};
use tokenizer::Tokenizer;
use parser::Parser;
use runner::Runner;

mod tokenizer;
mod parser;
mod runner;

fn main() {
    let mut args = std::env::args();

    let mut path = config();
    let mut out = None;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--sock" => {
                path = Some(args.next().expect("missing socket path after sock flag"));
            },
            "--out" => {
                out = Some(args.next().expect("missing socket path after sock flag"));
            },
            _ => {
                continue;
            }
        }
    }

    run(&path.unwrap(), out)
}

fn run(path: &str, out: Option<String>) {
    let tok = Tokenizer::new(path);

    let p = Parser::new(tok).parse();

    let mut groups = Vec::new();

    for group in p {
        groups.push(group);
    }

    let r = Runner::new(groups);

    if let Some(out) = out {
        r.write(&out);

        return
    }

    let sock = UnixListener::bind("/tmp/clts.sock").
        expect("socket already exists, probably the daemon is running");

    loop {
        let mut conn = sock.accept().unwrap();

        let mut input = String::new();

        conn.0.read_to_string(&mut input).unwrap();

        let args: Vec<String> = input.split(" ").map(|x| x.to_string()).collect();

        let _ = conn.0.write(r.map(args).join(" ").as_bytes());

        drop(conn);
    }
}

fn config() -> Option<String> {
    if path::Path::new("/etc/clts/clts.conf").exists() {
        return Some("/etc/clts/clts.conf".to_string())
    } 

    if let Ok(home) = std::env::var("HOME") {
        let config = home + "/.config/clts/clts.conf";
        if path::Path::new(&config).exists() {
            return Some(config)
        }
    }

    None
}
