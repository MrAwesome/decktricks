//use std::time::Duration;
//use std::thread;
//use decktricks::prelude::*;
//use std::sync::mpsc;
use decktricks::tricks_config::TricksConfig;
use clap::Parser;
use decktricks::actions::Cli;

//    let (sender, receiver) = mpsc::channel();
//
//    thread::spawn(move || {
//        let flatpak_name = receiver.recv();
//        let prov = Provider::flatpak("net.davidotek.pupgui2", true);
//        prov.is_runnable().and_then(|p| p.run());
//    });
//
//    thread::sleep(Duration::from_secs(1));
//    sender.send("net.davidotek.pupgui2");
//    println!("llllll");
//    thread::sleep(Duration::from_secs(4));

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = TricksConfig::from_default_config()?;
    let cli = Cli::parse();
    let action = &cli.command;

    action.run(config);
    
    Ok(())
}
