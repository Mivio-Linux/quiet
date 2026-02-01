
fn make_command(arguments: &Vec<String>, arg_num: usize) -> std::process::Output {
    
    let args_start = arg_num + 1;
    let command = std::process::Command::new(&arguments[arg_num]).args(&arguments[args_start..])
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .output().unwrap_or_else(|err| {
            eprintln!("[ERROR]: {}", err);
            std::process::exit(1);
        });
        command
}
fn print_errors_or_status_code(command: std::process::Output) {
    if !command.status.success() { 
            match command.status.code() {
                Some(code) =>{
                        let stderr =String::from_utf8_lossy(&command.stderr);
                        if !stderr.is_empty() {
                            eprintln!("[ERROR]: {stderr}");
                        } else {
                            eprintln!("[ERROR]: Exited with status code: {code}");
                        }
               },
                None => eprintln!("[ERROR]: Process terminated by signal")
            };
        }
}
fn log_in_file(file_path: &str, content: &str){
    let _file = std::fs::write(file_path, content);

}
fn main() {
    let print_usage_and_exit = ||{eprintln!("USAGE:
        Without log:
        quiet <command> [args]

        With log:
        quiet -l path/to/log.txt <command> [args]
        quiet --log path/to/log.txt <command> [args]
        
        Print this message:
        quiet -h
        quiet --help
        ");
        std::process::exit(1);};
    let arguments: Vec<String> = std::env::args().collect();
    if arguments.len() >= 2 {
        if arguments[1] == "-h" || arguments[1] == "--help" {
            print_usage_and_exit();
        }
        else if arguments[1] == "--log" || arguments[1] == "-l"{
            if arguments.len() < 4 {
                eprintln!("[ERROR]: Not enough arguments for --log");
                std::process::exit(1);
            }   
            let command = make_command(&arguments, 3);
            let out = String::from_utf8_lossy(&command.stdout);
            log_in_file(&arguments[2], &out);
            print_errors_or_status_code(command);
        } else {
            let command = make_command(&arguments, 1);
            print_errors_or_status_code(command);
        }
        
    } else {
        print_usage_and_exit();
    }
}