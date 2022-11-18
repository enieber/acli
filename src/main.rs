use std::process::exit;
use std::process::{Command, Stdio};
use std::str::from_utf8;
use terminal_menu::TerminalMenuItem;
use terminal_menu::{button, label, menu, mut_menu, run};

fn main() {
    let menu = menu(vec![
        label("-------------"),
        label("Android CLI"),
        label("use wasd or arrow keys"),
        label("enter to select"),
        label("'q' or esc to exit"),
        label("-----------------------"),
        button("Open Emulator"),
        button("Compile Android"),
        button("debug"),
    ]);
    run(&menu);

    if mut_menu(&menu).canceled() {
        println!("End..")
    } else {
        let item_selected = mut_menu(&menu).selected_item_name().to_string();
        match item_selected.as_ref() {
            "Open Emulator" => {
                emulator();
            }
            "Compile Android" => {
                compile_android();
            }
            "debug" => {
                build_apk_path();
            }
            _ => println!("fail"),
        }
    }
}

fn compile_android() {
    println!("Compile with gradle");
    Command::new("gradle").arg("build").status();
    install_app();
}

fn install_app() {
    println!("Install app");
    Command::new("gradle").arg("installDebug").status();
}

fn open_app() {
    println!("Open app installed");
    let apk_path = build_apk_path();
    Command::new("adb")
        .arg("shell")
        .arg("am")
        .arg("start")
        .arg("-n")
        .arg(&apk_path)
        .status();
}

fn build_apk_path() -> String {
    let path = "app/build/outputs/apk/debug/app-debug.apk";
    //aapt2 dump packagename
    let dump_apk = Command::new("aapt2")
        .arg("dump")
        .arg("packagename")
        .arg(&path)
        .output();
    let value = dump_apk.unwrap().stdout;
    let list_str = from_utf8(&value).unwrap().to_string();
    let val = format!("-v 1 -p {}", &list_str);

    Command::new("adb").args(["shell", "monkey", &val]).status();

    //package=$(aapt dump badging $apk_path | awk '/package/{gsub("name=|'"'"'","");  print $2}')
    //activity=$(aapt dump badging $apk_path | awk '/launchable-activity/{gsub("name=|'"'"'","");  print $2}')

    //echo "$package/$activity"
    return String::from("");
}

fn emulator() {
    println!("Emulators");
    let emulators = Command::new("emulator").arg("-list-avds").output();
    match emulators {
        Ok(emulator_list) => {
            let list_str = from_utf8(&emulator_list.stdout).unwrap();
            let list: Vec<&str> = list_str.split("/n").collect();

            let mut men = vec![
                label("----------------------"),
                label("Emulators"),
                label("use wasd or arrow keys"),
                label("enter to select"),
                label("'q' or esc to exit"),
                label("-----------------------"),
            ];

            for emulator in list {
                let emulator_str = emulator.replace("_", " ");
                let item = format!("{}", emulator_str);
                men.push(button(item));
            }

            let menu = menu(men);
            run(&menu);

            if mut_menu(&menu).canceled() {
                println!("End..")
            } else {
                let item_selected = mut_menu(&menu).selected_item_name().to_string();

                println!("Selected: {:?}", &item_selected);

                Command::new("emulator")
                    .arg("-netdelay")
                    .arg("none")
                    .arg("-netspeed")
                    .arg("full")
                    .arg("-avd")
                    .arg(&item_selected.replace("\n", "").replace(" ", "_"))
                    .status()
                    .expect("failed to execute process");
            }
        }
        Err(err) => {
            println!("Error to running emulator: {:?}", err);
        }
    }
}
