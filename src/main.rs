use std::io::*;
use std::net::TcpStream;
use dotenvy::dotenv;
use std::env;

use anyhow::Result;
//use tokio;

const TWITCH_IRC_ADDRESS: &str = "irc.twitch.tv:6667";
const BUFFER_SIZE: usize = 2048; 

fn build_command<'a>(command: &'a str, body: &'a str) -> String {
    format!("{command} {body}\r\n")
}

fn authenticate(tcp_stream: &mut TcpStream) -> Result<()> {
    // TODO: read Oauth token from .env and insert into PASS command
    tcp_stream.write(build_command("PASS", &env::var("OAUTH_TOKEN")?).as_bytes())?;
    tcp_stream.write(build_command("NICK", "hicaro____").as_bytes())?;
    Ok(())
}

fn main() -> Result<()> {
    dotenv().ok();

    let mut stream = TcpStream::connect(TWITCH_IRC_ADDRESS)?;
    let mut buffer = [0; BUFFER_SIZE];
    authenticate(&mut stream)?;
    loop {
        stream.read(&mut buffer)?;
        let message = std::str::from_utf8(&buffer)?;
        println!("{}", message);
    }
}
