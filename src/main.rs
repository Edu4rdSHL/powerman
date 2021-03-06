use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process::Command;
//extern crate schedule_recv;
use notify_rust::Notification;

fn main() {
    // Get user arguments, first argument is the level of battery
    // when the user want to take a action, second argument is the
    // action to take using systemctl. For example, if you want to
    // hibernate when the battery level is less than 5%, you should
    // pass the following aguments:  5 hibernate
    let args: Vec<String> = env::args().collect();

    if args.len() == 4 {
        let bat_discharging_action: u32 = args[1]
            .trim()
            .parse()
            .expect("Error, converting minimun level of battery to a valid number.");

        // Convert user agument into integer
        let execution_time: u32 = args[2]
            .trim()
            .parse()
            .expect("Error, execution time argument can not be converted in a valid number.");

        println!("You are going to {} your system if the battery level is equal or less to {}% and the check is done every {} seconds.", &args[3], bat_discharging_action, execution_time);

        // Execution time need to be miliseconds
        let execution_time = execution_time * 1000;
        let tick = schedule_recv::periodic_ms(execution_time);

        let battery_charging_path = "/sys/class/power_supply/BAT0/status";

        let mut notification_status = String::from("Not runned");
        loop {
            tick.recv().unwrap();

            let mut bat_charging = String::new();
            let mut b_status = File::open(&battery_charging_path).expect("Error opening file.");

            b_status
                .read_to_string(&mut bat_charging)
                .expect("Failed to read file.");

            battery_critical_action(bat_discharging_action, &bat_charging, &args[3]);

            if bat_charging.trim() == "Discharging" {
                while notification_status == "Not runned" {
                    notification_status = battery_notification(&bat_charging);
                    break;
                }
            } else if bat_charging.trim() == "Charging" && notification_status != "Not runned" {
                Notification::new()
                    .summary("Battery charging")
                    .body("Battery is charging now.")
                    .icon("battery-full-charging-symbolic")
                    .timeout(0)
                    .show()
                    .unwrap();
                notification_status = String::from("Not runned")
            }
        }
    } else {
        println!("Usage:
    powerman <Minimun level of battery before ation> <Time in seconds to check the battery level> <Action>
    Example:

    powerman 5 60 hibernate
    - Hibernate the computer when the battery level is less than 5%, the check is done every 60 seconds.
    ");
    }
}

fn battery_critical_action(percentage: u32, bat_charging: &str, action: &str) {
    // Path for battery capacity percentage, battery charging status and systemd
    let battery_capacity_path = "/sys/class/power_supply/BAT0/capacity";
    let systemctl_path = "/usr/bin/systemctl";

    // Check if systemd and BAT0 device exist
    if Path::new(&battery_capacity_path).exists() && Path::new(&systemctl_path).exists() {
        // Declare variables
        let mut bat_value = String::new();

        // Read the capacity from file
        let mut b_capacity = File::open(&battery_capacity_path).expect("Error opening file.");

        // Convert percentage into string
        b_capacity
            .read_to_string(&mut bat_value)
            .expect("Failed to read file.");

        // Convert percentage into integer
        let bat_percentage: u32 = bat_value
            .trim()
            .parse()
            .expect("Error, battery percentage level can not be converted in a valid number.");

        // Compare percentagei, status and execute actions accordy to use input
        if bat_percentage <= percentage && bat_charging.trim() == "Discharging" {
            let command_action = Command::new(&systemctl_path)
                //Get user argument 2 for action to execute
                .arg(&action)
                .output()
                .expect("Failed to execute command.");

            if command_action.status.success() == false {
                eprintln!("Process exited with: {}", &command_action.status);
                eprintln!(
                    "\nMore information about the error:\n\n {:#?}",
                    &command_action
                );
            }
        }
    }
}

fn battery_notification(bat_charging: &str) -> String {
    // Path for battery capacity percentage, battery charging status and systemd
    let battery_capacity_path = "/sys/class/power_supply/BAT0/capacity";

    let percentage_crit = 10;

    // Check if systemd and BAT0 device exist
    if Path::new(&battery_capacity_path).exists() {
        // Declare variables
        let mut bat_value = String::new();

        // Read the capacity from file
        let mut b_capacity = File::open(&battery_capacity_path).expect("Error opening file.");

        // Convert percentage into string
        b_capacity
            .read_to_string(&mut bat_value)
            .expect("Failed to read file.");

        // Convert percentage into integer
        let bat_percentage: u32 = bat_value
            .trim()
            .parse()
            .expect("Error, battery percentage level can not be converted in a valid number.");

        if bat_percentage <= percentage_crit && bat_charging.trim() == "Discharging" {
            Notification::new()
                .summary("Battery level critical (~10%)")
                .body("Please connect a charguer.")
                .icon("battery-empty-symbolic")
                .timeout(0)
                .show()
                .unwrap();
            String::from("Critical")
        } else {
            String::from("Not runned")
        }
    } else {
        String::from("Some paths doesn't exist.")
    }
}
