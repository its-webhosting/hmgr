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

fn test_mac_perms() -> std::io::Result<()> {
    let file = read_to_string("/etc/hosts")?;
    // Try to write to file to check perms
    write("/etc/hosts", file)?;
    Ok(())
}

fn test_win_perms() -> std::io::Result<()> {
    let file = read_to_string("C:\\Windows\\System32\\drivers\\etc\\hosts")?;
    // Try to write to file to check perms
    write("C:\\Windows\\System32\\drivers\\etc\\hosts", file)?;
    Ok(())
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
    // Promise to check file perms
    if cfg!(target_os = "macos") {
        println!("Saving requires administrative permissions... Skipping for now.");
    } else if cfg!(target_os = "linux") {
        // linux_perms().unwrap();
    } else if cfg!(target_os = "windows") {
        test_win_perms().unwrap_or_else(|_| {
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
        });

        // Make a backup of the hosts file
        let hosts_backup = read_to_string("C:\\Windows\\System32\\drivers\\etc\\hosts").unwrap();
        // Name file based on date and time
        let hosts_backup_name = format!(
            ".\\hosts-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs()
        );
        write(hosts_backup_name, hosts_backup).unwrap();
    }
    // Promise Success
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, argv, cwd| {
            println!("{}, {argv:?}, {cwd}", app.package_info().name);

            app.emit_all("single-instance", Payload { args: argv, cwd })
                .unwrap();
        }))
        .invoke_handler(tauri::generate_handler![open_hosts, save_hosts])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    // Promise Fail
    exit(0);
}
