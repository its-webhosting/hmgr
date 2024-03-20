// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs::read_to_string;
use std::fs::write;
use std::process::exit;

use tauri::Manager;

#[derive(Clone, serde::Serialize)]
struct Payload {
    args: Vec<String>,
    cwd: String,
}

#[tauri::command]
fn open_hosts() {
    if cfg!(target_os = "macos") {
        let _ = std::process::Command::new("open").arg("/etc/hosts").spawn();
    } else if cfg!(target_os = "linux") {
        let _ = std::process::Command::new("open").arg("/etc/hosts").spawn();
    } else if cfg!(target_os = "windows") {
        let _ = std::process::Command::new("notepad")
            .arg("C:\\Windows\\System32\\drivers\\etc\\hosts")
            .spawn();
    }
}

#[tauri::command]
fn save_hosts(hosts_string: &str) {
    if cfg!(target_os = "macos") {
        std::process::Command::new("osascript")
            .arg("-e")
            .arg(format!(
                "do shell script \"echo '{}' > /etc/hosts\" with administrator privileges",
                hosts_string
            ))
            .spawn()
            .unwrap();
    } else if cfg!(target_os = "linux") {
        std::process::Command::new("pkexec")
            .arg("bash")
            .arg("-c")
            .arg(format!("echo '{}' > /etc/hosts", hosts_string))
            .spawn()
            .unwrap();
    } else if cfg!(target_os = "windows") {
        std::fs::write("C:\\Windows\\System32\\drivers\\etc\\hosts", hosts_string).unwrap();
    }
}

fn main() {
    // Set File Location based on OS
    let etc_file_loc = if cfg!(target_os = "windows") {
        "C:\\Windows\\System32\\drivers\\etc\\hosts"
    } else {
        "/etc/hosts"
    };
    let hosts_backup: &str = &read_to_string(etc_file_loc).unwrap();
    // Create a backup based on the contents of the file
    if let Ok(file) = read_to_string(etc_file_loc) {
        if !file.starts_with("#N Managed by Hosts Manager") {
            // If not, create a backup
            let hosts_backup_name = format!(
                ".\\hosts-{}",
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs()
            );
            write(hosts_backup_name, hosts_backup).unwrap();
        }
    }
    // try to write, fix file perms if fails
    write(etc_file_loc, hosts_backup).unwrap_or_else(|_| {
        if cfg!(target_os = "windows") {
            std::process::Command::new("powershell")
                .arg("Start-Process")
                .arg("cmd.exe")
                .arg("-ArgumentList")
                .arg("'/c icacls \"C:\\Windows\\System32\\drivers\\etc\\hosts\" /grant Everyone:F'")
                .arg("-Verb")
                .arg("RunAs")
                .spawn()
                .unwrap_or_else(|_| {
                    println!("Failed to Fix File Permissions! Exiting...");
                    exit(1);
                });
        } else if cfg!(target_os = "mac") {
            std::process::Command::new("osascript")
                .arg("-e")
                .arg(format!(
                    "do shell script \"chmod 664 /etc/hosts\" with administrator privileges"
                ))
                .spawn()
                .unwrap_or_else(|_| {
                    println!("Failed to Fix File Permissions! Exiting...");
                    exit(1);
                });
        } else if cfg!(target_os = "linux") {
            println!("Not implemented yet!");
            exit(1);
        }
        tauri::Builder::default()
            .plugin(tauri_plugin_single_instance::init(|app, argv, cwd| {
                println!("{}, {argv:?}, {cwd}", app.package_info().name);

                app.emit_all("single-instance", Payload { args: argv, cwd })
                    .unwrap();
            }))
            .invoke_handler(tauri::generate_handler![open_hosts, save_hosts])
            .run(tauri::generate_context!())
            .expect("error while running tauri application");
    });
}
