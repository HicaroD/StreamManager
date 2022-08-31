use dotenvy::dotenv;
use std::env;
use std::io::*;
use std::net::TcpStream;

use anyhow::Result;
//use tokio;

const TWITCH_IRC_ADDRESS: &str = "irc.twitch.tv:6667";
const BUFFER_SIZE: usize = 2048;

fn build_command<'a>(command: &'a str, body: &'a str) -> String {
    format!("{command} {body}\r\n")
}

fn authenticate(tcp_stream: &mut TcpStream) -> Result<()> {
    tcp_stream.write(build_command("PASS", &env::var("OAUTH_TOKEN")?).as_bytes())?;
    tcp_stream.write(build_command("NICK", "hicaro____").as_bytes())?;
    Ok(())
}

fn join_channel(tcp_stream: &mut TcpStream) -> Result<()> {
    let command = build_command("JOIN", &format!("#{}", env::var("CHANNEL")?));
    tcp_stream.write(command.as_bytes())?;
    Ok(())
}

fn main() -> Result<()> {
    dotenv().ok();

    let mut stream = TcpStream::connect(TWITCH_IRC_ADDRESS)?;
    let mut buffer = [0; BUFFER_SIZE];
    authenticate(&mut stream)?;
    join_channel(&mut stream)?;
    loop {
        stream.read(&mut buffer)?;
        let message = std::str::from_utf8(&buffer)?;
        println!("{}", message);
    }
}
