use std::io::{BufReader,prelude::*};
use std::net::TcpStream;
use colored::*;
use std::io::{Error,ErrorKind};
pub fn read_from_stream(stream: &TcpStream)-> Result<String, std::io::Error> {
    let buf_reader = BufReader::new(stream);
    let received_data = buf_reader
                                        .lines()
                                        .map(|line| line.unwrap())                                            
                                        .take_while(|line| !line.is_empty())
                                        .collect::<Vec<_>>().join("\n");
    Ok(received_data)
}


pub fn send_to_steam(stream: &mut TcpStream, msg: &str) -> Result<(), std::io::Error> {
    let data = msg.to_string() + "\n\n";
    stream.write_all(data.as_bytes())
}

pub fn authenticate(stream: &mut TcpStream, user: &str, pass: &str) -> Result<bool, std::io::Error> {
    let data = format!("{} {}\n\n", user, pass);
    send_to_steam(stream,data.as_str())?;

    let response = read_from_stream(stream)?;
    Ok(response == "OK")
}

pub fn edit_line(stream: &mut TcpStream) -> Result<(), std::io::Error> {
    let nth_line = read_from_stream(stream).unwrap();
    if nth_line == "File not found" || nth_line == "Line number out of bounds" || 
    nth_line == "Invalid line number" || nth_line == "Line number not provided" ||
    nth_line == "No filename provided" || nth_line == "Line number must be greater than 0" {
        return Err(Error::new(ErrorKind::Other, nth_line));
    }
    println!("{} {}","Requested line:".to_string().yellow(),nth_line);
    print!("🦀 {}","Enter new line: ".to_string().magenta());
    std::io::stdout().flush().unwrap();
    let mut new_line = String::new();
    std::io::stdin().read_line(&mut new_line).unwrap();
    new_line = new_line.trim().to_string();
    send_to_steam(stream, &new_line)?;
    let res = read_from_stream(stream)?;
    println!("{}",res);
    Ok(())
}