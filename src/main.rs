

fn main() {
    let arguments: Vec<String> = std::env::args().collect();
    // println!("{:?}",arguments);
    if arguments.len() >= 2 {
        //let error: String;
        let command = std::process::Command::new(&arguments[1]).args(&arguments[2..])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::piped())
        .output().unwrap_or_else(|e| {
            eprintln!("[ERROR]: {}", e);
            std::process::exit(1);
        });
        if !command.status.success() {
            print!("[ERROR]: {}", String::from_utf8_lossy(&command.stderr));
        }
    }
}