use std::{
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
    let tok = Tokenizer::new("test.txt");

    let p = Parser::new(tok).parse();

    let mut groups = Vec::new();

    for group in p {
        groups.push(group);
    }

    let r = Runner::new(groups);

    let sock = UnixListener::bind("/tmp/clts.sock").
        expect("socket does not exist, check whether the daemon is running");

    loop {
        let mut conn = sock.accept().unwrap();

        let mut input = String::new();

        conn.0.read_to_string(&mut input).unwrap();

        let args: Vec<String> = input.split(" ").map(|x| x.to_string()).collect();

        let _ = conn.0.write(r.map(args).join(" ").as_bytes());

        drop(conn);
    }
}
