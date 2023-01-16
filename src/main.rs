use std::net::TcpStream;
use colored::Colorize;
use tcp_client::{authenticate,send_to_steam,read_from_stream,edit_line};
use std::io::{self,Write};

fn main() {    
    match TcpStream::connect("192.168.1.9:3333") {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 3333");

            let mut username = String::new();            

            print!("😄 Enter username: ");
            io::stdout().flush().unwrap();
            std::io::stdin().read_line(&mut username).unwrap();
            username = username.trim().to_string();

            print!("🦆 Enter password: ");
            io::stdout().flush().unwrap();
            let mut password = rpassword::read_password().unwrap();
            password = password.trim().to_string();

            if authenticate(&mut stream, &username, &password).unwrap() {
                println!("🎊 {}","Authentication Successfully".to_string().green());
                loop{
                    let mut req = String::new();
                    print!("🦀 {}","Enter your request: ".to_string().magenta());
                    io::stdout().flush().unwrap();
                    std::io::stdin().read_line(&mut req).unwrap();
                    req = req.trim().to_string();

                    let subcommand = req.split(" ").next().unwrap();

                    if req != "ls" && subcommand != "cat" && subcommand != "edit" && req != "exit" {
                            eprintln!("🤔 {}","Invalid subcommand".to_string().red());
                            continue;
                    }

                    match send_to_steam(&mut stream, &req){
                        Ok(()) => {},
                        Err(e) => {
                            eprintln!("Error: {}", e);
                            break;
                        }
                    };

                    if req == "exit" {
                        println!("👋 Exiting...");
                        break;
                    }else if subcommand == "edit"{
                        if let Err(e) = edit_line(&mut stream) {
                            eprintln!("💥Error: {}",e.to_string().red());
                        }
                    }else{
                        let response = read_from_stream(&mut stream).unwrap();
                        println!("\n{} \n{}","Response:".to_string().cyan(), response);
                    }
                    println!();
                }
            } else {
                println!("👎 {}","Authentication failed".to_string().red());
            }
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    println!("🙋 Terminated.");
}