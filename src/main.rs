#[macro_use]
extern crate chan;
extern crate notify_rust;
use notify_rust::Notification;


fn main() {
    // Run once
    batterup();

    // Setup recuring timer
    let timer = chan::tick_ms(60000);
    loop {
        chan_select! {
            timer.recv() => batterup(),
        }
    }
}

fn batterup(){
    use std::process::Command;

    let charge_output = Command::new("cat")
                         .arg("/sys/class/power_supply/BAT0/capacity")
                         .output()
                         .expect("Battery not found. Are you on a laptop?");

    let charge_stdout = String::from_utf8_lossy(&charge_output.stdout);

    let charge_vec = charge_stdout.split("\n").collect::<Vec<&str>>();

    let charge =  charge_vec[0].parse::<i32>().unwrap();

    let status_output = Command::new("cat")
                         .arg("/sys/class/power_supply/BAT0/status")
                         .output()
                         .expect("Battery not found. Are you on a laptop?");

    let stdout = String::from_utf8_lossy(&status_output.stdout);

    let vec = stdout.split("\n").collect::<Vec<&str>>();

    let status = vec[0];

    if charge >= 80 && status != "Discharging"&& charge % 5 == 0 {
        // Notify to unplug
        Notification::new()
            .summary("Battery Charged")
            .body(&format!("Battery at {}%! Unplug your laptop!", &charge))
            .icon("battery")
            .appname("Batter Up!")
            .timeout(0) // this however is
            .show().unwrap();
    }
    else if charge <= 20 && status != "Charging" && charge % 5 == 0 {
        // Notify to plug-in
        Notification::new()
            .summary("Battery Depleted")
            .body(&format!("Battery at {}%! Plug in your laptop!", &charge))
            .icon("battery")
            .appname("Batter Up!")
            .timeout(0) // this however is
            .show().unwrap();
    }

}

