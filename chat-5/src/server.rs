use serde::Serialize;
use serde_json;
use std::{
    collections::HashMap,
    io::{self, BufRead, BufReader, BufWriter},
    net::{TcpListener, TcpStream},
    thread,
};

#[derive(Clone)]
pub struct Server {
    port: u32,
}

impl Server {
    pub fn new(port: u32) -> Server {
        Server { port }
    }

    pub fn run(&self) -> io::Result<()> {
        let addr = format!("0.0.0.0:{}", self.port);
        let listener = TcpListener::bind(addr)?;
        for stream in listener.incoming() {
            let stream = stream?;
            let server = self.clone();
            thread::spawn(move || server.handle(stream));
        }
        Ok(())
    }

    fn handle(&self, mut stream: TcpStream) {
        stream = dbg!(stream);
        let reader = BufReader::new(stream.try_clone().unwrap());
        let writer = BufWriter::new(stream);
        self.to_bs().unwrap();
    }

    fn to_bs(&self) -> io::Result<()> {
        let mut m = HashMap::new();
        m.insert("port", self.port);
        let mut writer = vec![];
        let mut serializer = serde_json::Serializer::new(&mut writer);
        m.serialize(&mut serializer)?;
        dbg!(m);
        Ok(())
    }
}
