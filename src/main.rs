use core::time;
use std::{process::Command, thread::sleep, time::Duration};

use sysinfo::{ProcessExt, Signal, System, SystemExt};

fn main() {
    //Get commandline args
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        println!("Usage: {} <process_name> <epic_url>", args[0]);
        return;
    }
    let game_name = args.get(1).unwrap();
    let game_url = args.get(2).unwrap();
    open_url(&game_url);

    let one_second = time::Duration::from_secs(1);

    let mut process_id = get_process_id_by_name(game_name.as_str());
    while process_id.is_none() {
        println!("Waiting for process to be created");
        sleep(one_second);
        process_id = get_process_id_by_name(game_name.as_str());
    }
    println!(
        "Found game process, with id: {} , now wait for it to be gone",
        process_id.unwrap()
    );
    if let Some(pid) = process_id {
        let mut s = System::new();
        while s.refresh_process(pid) {
            sleep(one_second);
        }
    }
    println!("Game is closed, now closing laucher");
    let s = System::new_all();
    let mut launcher_process_id = s.process_by_name("EpicGamesLauncher.exe");
    while launcher_process_id.is_empty() {
        sleep(Duration::from_secs(1));
        launcher_process_id = s.process_by_name("EpicGamesLauncher.exe");
    }
    if let Some(process) = launcher_process_id.iter().next() {
        println!("Found launcher");
        let mut s = System::new();
        process.kill(Signal::Kill);
        process.kill(Signal::Quit);
        println!("Sending kill");
        while s.refresh_process(process.pid()) {
            println!("Waiting for launcher to die");
            sleep(Duration::from_secs(1));
        }
    }
}

fn get_process_id_by_name(name: &str) -> Option<usize> {
    let s = System::new_all();
    let processes = s.process_by_name(&name);
    processes.iter().map(|p| p.pid()).next()
}

fn open_url(url: &str) -> bool {
    if let Ok(mut child) = Command::new("cmd.exe")
        .arg("/C")
        .arg("start")
        .arg("")
        .arg(&url)
        .spawn()
    {
        sleep(time::Duration::new(1, 0));
        if let Ok(status) = child.wait() {
            return status.success();
        }
    }
    false
}
