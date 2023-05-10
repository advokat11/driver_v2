    use std::path::Path;
    use std::process::Command;
    use walkdir::{DirEntry, WalkDir};
    use indicatif::{ProgressBar, ProgressStyle};
    fn main() {
        println!("==========================================");
        println!("=============== AQ Drivers ===============");
        println!("==========================================");
        println!("\nWarning: Do not turn off your computer during the installation process.\n");    
        let start_path = ".";
    
        let inf_files_count = count_inf_files(start_path);
        let progress_bar = ProgressBar::new(inf_files_count as u64);
        progress_bar.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
                .progress_chars("#>-"),
        );
    
        for entry in WalkDir::new(start_path).into_iter().filter_map(|e| e.ok()) {
            if is_inf_file(&entry) {
                let path = entry.path();
                install_inf_driver(&path);
    
                progress_bar.inc(1);
            }
        }
    
        progress_bar.finish_and_clear();
        println!("Установка драйверов завершена.");
    }
    
    fn is_inf_file(entry: &DirEntry) -> bool {
        entry
            .file_name()
            .to_str()
            .map_or(false, |s| s.ends_with(".inf"))
            && entry.file_type().is_file()
    }
    
    fn count_inf_files(start_path: &str) -> usize {
        WalkDir::new(start_path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|entry| is_inf_file(entry))
            .count()
    }
    
    fn install_inf_driver(inf_path: &Path) {
        let _ = Command::new("pnputil")
            .arg("/add-driver")
            .arg(inf_path)
            .arg("/install")
            .output();
    }
    