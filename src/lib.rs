pub mod applications;

use std::{io::{BufRead, BufReader}, process::{Command, Stdio}};

fn run_command(id: &str) -> bool {
    let mut binding = Command::new("winget");

    let command = binding
        .arg("install")
        .arg("--id")
        .arg(id);

    let mut child= command.stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to execute winget list command");   

    if let Some(stdout) = child.stdout.take() {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            if let Ok(line) = line {
                println!("stdout: {}", line);
            }
        }
    }

    if let Some(stderr) = child.stderr.take() {
        let reader = BufReader::new(stderr);
        for line in reader.lines() {
            if let Ok(line) = line {
                println!("stderr: {}", line);
            }
        }
    }

    let status = child.wait().expect("Command wasn't running");

    status.success()
}

fn is_app_installed(app_id: &str) -> bool {
    let output = Command::new("winget")
        .arg("list")
        .arg("--id")
        .arg(app_id)
        .output()
        .expect("Failed to execute winget list command");
    let output = output;

    let stdout = String::from_utf8_lossy(&output.stdout);

    stdout.contains(app_id)
}

pub fn download_application(name: &str, id: &str){
    println!("Checking if {} is download", name);
    if !is_app_installed( id) {
        if run_command(id) {
            
            println!("Installed {} [✔]", name);
        }
    } else {
        println!("{} is already installed [✔]", name);
    }
}