use indicatif::{ProgressBar, ProgressStyle};
use std::borrow::Cow;
use std::env;
use std::sync::{atomic::{AtomicUsize, Ordering}, Arc};
use std::process::{Command, ExitStatus, Stdio};
use walkdir::{DirEntry, WalkDir};
use std::io::{Write};
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

    let (term_width, _) = term_size::dimensions().unwrap_or((80, 20));

    let pb = ProgressBar::new(num_drivers);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta}) {msg}")
        .progress_chars("#>-"));

    let successful_installs = Arc::new(AtomicUsize::new(0));
    let failed_installs = Arc::new(AtomicUsize::new(0));

    let log_file = if logging_enabled {
        Some(File::create("log.txt").unwrap())
    } else {
        None
    };

    driver_list.into_iter().for_each(|entry| {
        let path = entry.path();
        let driver_name = path.file_name().unwrap().to_string_lossy();
        let message = format!("Installing driver: {}", driver_name);
        pb.set_message(Cow::Owned(message.clone()));

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

        print!("{:<1$}\r", "", term_width as usize);
        std::io::stdout().flush().unwrap();

        pb.inc(1);
    });

    pb.finish_with_message("Done");

    println!(
        "Successful installs: {}, Failed installs: {}",
        successful_installs.load(Ordering::SeqCst),
        failed_installs.load(Ordering::SeqCst),
    );
}