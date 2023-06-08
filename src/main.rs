use pbr::ProgressBar;
use std::env;
use std::sync::{atomic::{AtomicUsize, Ordering}, Arc};
use std::process::{Command, ExitStatus, Stdio};
use walkdir::{DirEntry, WalkDir};
use std::fs::File;

#[cfg(unix)]
use std::os::unix::process::ExitStatusExt;

#[cfg(windows)]
use std::os::windows::process::ExitStatusExt;

fn main() {
    let args: Vec<String> = env::args().collect();

    let logging_enabled = args.len() > 1 && args[1] == "log";

    let driver_list: Vec<DirEntry> = WalkDir::new(".")
    .into_iter()
    .filter_map(Result::ok)
    .filter(|e| e.file_name().to_string_lossy().to_lowercase().ends_with(".inf"))
    .collect();

    let num_drivers = driver_list.len() as u64;

    let mut pb = ProgressBar::new(num_drivers);

    let successful_installs = Arc::new(AtomicUsize::new(0));
    let failed_installs = Arc::new(AtomicUsize::new(0));

    let log_file = if logging_enabled {
        Some(File::create("log.txt").unwrap())
    } else {
        None
    };

    for entry in driver_list.into_iter() {
        let path = entry.path();
        let driver_name = path.file_name().unwrap().to_string_lossy();
        let message = format!("Installing driver: {} ", driver_name);
        pb.message(&message);

        let mut command = Command::new("pnputil.exe");
        command.arg("/add-driver")
            .arg(path.as_os_str())
            .arg("/install");

        if let Some(log_file) = &log_file {
            command.stdout(Stdio::from(log_file.try_clone().unwrap()))
                   .stderr(Stdio::from(log_file.try_clone().unwrap()));
        } else {
            command.stdout(Stdio::null())
                   .stderr(Stdio::null());
        }

        let status = command.status().unwrap_or_else(|_| ExitStatus::from_raw(1));

        if status.success() {
            successful_installs.fetch_add(1, Ordering::SeqCst);
        } else {
            failed_installs.fetch_add(1, Ordering::SeqCst);
        }

        pb.inc();
    }

    pb.finish_print("Done");

    println!(
        " Successful installs: {}, Failed installs: {}",
        successful_installs.load(Ordering::SeqCst),
        failed_installs.load(Ordering::SeqCst),
    );
}
