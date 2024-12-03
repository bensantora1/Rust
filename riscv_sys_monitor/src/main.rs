use sysinfo::{System, SystemExt, CpuExt, DiskExt};
use chrono::Local;
use std::{thread, time::Duration};
use structopt::StructOpt;
use std::collections::HashSet;

#[derive(StructOpt)]
#[structopt(name = "riscv_sysmon", about = "System monitor for RISC-V SBCs on Linux")]
struct Cli {
    #[structopt(short = "i", long = "interval", default_value = "1", help = "Update interval in seconds")]
    interval: u64,
}

fn main() {
    let args = Cli::from_args();
    let update_interval = Duration::from_secs(args.interval);

    println!("Starting riscv_sysmon...");
    let mut sys = match System::new_all() {
        sys => sys,
    };

    loop {
        match refresh_system(&mut sys) {
            Ok(_) => print_system_info(&sys),
            Err(e) => eprintln!("Error refreshing system info: {}", e),
        }
        thread::sleep(update_interval);
    }
}

/// Refreshes all system information, returns an error if the refresh fails
fn refresh_system(sys: &mut System) -> Result<(), String> {
    sys.refresh_all();
    Ok(())
}

/// Formats bytes into human-readable sizes (e.g., MB, GB, TB)
fn format_size(size_in_bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    const TB: u64 = GB * 1024;

    if size_in_bytes >= TB {
        format!("{:.2} TB", size_in_bytes as f64 / TB as f64)
    } else if size_in_bytes >= GB {
        format!("{:.2} GB", size_in_bytes as f64 / GB as f64)
    } else if size_in_bytes >= MB {
        format!("{:.2} MB", size_in_bytes as f64 / MB as f64)
    } else {
        format!("{:.2} KB", size_in_bytes as f64 / KB as f64)
    }
}

/// Prints CPU, memory, and disk information from the system
fn print_system_info(sys: &System) {
    let local_time = Local::now();
    println!("\nSystem Metrics - {}", local_time.format("%Y-%m-%d %H:%M:%S"));
    println!("---------------------------");

    // CPU Information
    let cpu_usage: f32 = sys.cpus().iter().map(|cpu| cpu.cpu_usage()).sum::<f32>() / sys.cpus().len() as f32;
    println!("CPU Usage: {:.2}%", cpu_usage);

    // Memory Information
    let total_memory = sys.total_memory();
    let used_memory = sys.used_memory();
    println!("Memory Usage: {} / {} MB", used_memory / 1024, total_memory / 1024);

    // Disk Usage Information
    println!("\nDisk Usage:");
    let mut seen_disks = HashSet::new(); // To track and avoid duplicates
    let mut total_used = 0u64;
    let mut total_free = 0u64;
    let mut total_space = 0u64;

    for disk in sys.disks() {
        let name = disk.name().to_string_lossy();
        let total_space_mb = disk.total_space();
        let available_space_mb = disk.available_space();
        let used_space_mb = total_space_mb - available_space_mb;

        // Skip ambiguous or duplicate disks
        if name != "none" && total_space_mb > 0 && seen_disks.insert((name.clone(), total_space_mb)) {
            total_used += used_space_mb;
            total_free += available_space_mb;
            total_space += total_space_mb;

            println!(
                "Disk: {:<15} | Used: {:>8} | Free: {:>8} | Total: {:>8}",
                name,
                format_size(used_space_mb),
                format_size(available_space_mb),
                format_size(total_space_mb),
            );
        }
    }

    // Summary of total disk usage
    println!(
        "\nTotal Disk Usage: Used: {} | Free: {} | Total: {}",
        format_size(total_used),
        format_size(total_free),
        format_size(total_space),
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use sysinfo::{System, SystemExt};

    #[test]
    fn test_cpu_usage_calculation() {
        let mut sys = System::new_all();
        sys.refresh_cpu();

        let cpu_usage: f32 = sys.cpus().iter().map(|cpu| cpu.cpu_usage()).sum::<f32>() / sys.cpus().len() as f32;
        assert!(cpu_usage >= 0.0 && cpu_usage <= 100.0, "CPU usage should be between 0 and 100%");
    }

    #[test]
    fn test_memory_usage_retrieval() {
        let mut sys = System::new_all();
        sys.refresh_memory();

        let total_memory = sys.total_memory();
        let used_memory = sys.used_memory();
        assert!(total_memory >= used_memory, "Total memory should be greater than or equal to used memory");
    }

    #[test]
    fn test_disk_usage_retrieval() {
        let mut sys = System::new_all();
        sys.refresh_disks();

        for disk in sys.disks() {
            let total_space = disk.total_space();
            let available_space = disk.available_space();
            assert!(total_space >= available_space, "Total space should be greater than or equal to available space");
        }
    }
}
