use std::env;
use std::process::Command;
use walkdir::WalkDir;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::borrow::Cow;

fn main() {
    println!("==========================================");
    println!("=============== AQ Drivers ===============");
    println!("==========================================");
    println!("\nWarning: Do not turn off your computer during the installation process.\n");
    
    let current_dir = env::current_dir().unwrap();
    let walker = WalkDir::new(&current_dir).into_iter();
    
    let inf_files: Vec<_> = walker
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_name().to_string_lossy().ends_with(".inf"))
        .collect();

    let total_files = inf_files.len() as u64;
    
    let pb = ProgressBar::new(total_files);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos:>7}/{len:7} {msg}")
        .progress_chars("#>-"));

    inf_files.par_iter().for_each(|entry| {
        let path = entry.path();
        let message = format!("Устанавливаем драйвер: {:?}", path);

        pb.set_message(Cow::Owned(message));
        pb.inc(1);

        let _ = Command::new("pnputil.exe")
            .arg("/add-driver")
            .arg(path.as_os_str())
            .arg("/install")
            .output();
    });

    pb.finish_with_message("Завершено");
}
