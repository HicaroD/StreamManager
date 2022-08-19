use std::io::*;
use std::collections::HashMap;
use std::net::{TcpListener, TcpStream};

use anyhow::Result;
//use tokio;

const TWITCH_IRC_ADDRESS: &str = "irc.twitch.tv:443";
const BUFFER_SIZE: usize = 2048; 

fn main() -> Result<()> {
    let mut irc_connection = TcpStream::connect(TWITCH_IRC_ADDRESS)?;
    let mut buffer = [0; BUFFER_SIZE];
    loop {
        irc_connection.read(&mut buffer)?;
        let message = std::str::from_utf8(&buffer)?;
        println!("{}", message);
    }
}
