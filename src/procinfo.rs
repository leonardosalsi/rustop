use procfs::process::{Stat, Status, StatM};
use procfs::Uptime;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

// Process information
pub struct ProcessInfo {
    pid:     String,
    user:    String,
    virt:    String,
    res:     String,
    shr:     String,
    status:  String,
    cpu:     String,
    mem:     String,
    time:    String,
    command: String,
}

// CPU usage calculated according to https://www.baeldung.com/linux/total-process-cpu-usage
fn get_cpu_usage(stat: &Stat) -> String {
    let utime = stat.utime / procfs::ticks_per_second(); // CPU time spent in user space in seconds
    let stime = stat.stime / procfs::ticks_per_second(); // CPU time spent in system space in seconds
    let Uptime { uptime, .. } = Uptime::new().unwrap(); // System uptime in seconds
    let process_elapsed_sec = uptime - (stat.starttime / procfs::ticks_per_second()) as f64; //Elapsed time in seconds
    let cpu_usage = 100.0 * ((utime + stime) as f64 / process_elapsed_sec) as f32;
    return format!("{:.2}", cpu_usage);
}

//Get memory usage directly from corresponding statm file (due to manual computation being to slow currently)
fn get_mem_usage(statm: &StatM, pid: i32) -> Result<(String, String, String, String), std::io::Error> {
    let statm_path = format!("/proc/{}/statm", pid);
    let statm_content = fs::read_to_string(&statm_path)?;

    let fields: Vec<u64> = statm_content
        .split_whitespace()
        .filter_map(|s| s.parse::<u64>().ok())
        .collect();

    fn format_number(num: u64) -> String {
        if num > 999999 {
            let formatted = format!("{:.2e}", num);
            return formatted;
        } else {
            return num.to_string();
        }
    }

    if fields.len() >= 3 {
        let virt = fields[0];  // Virtual memory size (VIRT)
        let res = fields[1];   // Resident set size (RES)
        let shr = fields[2];   // Shared memory size (SHR)
        let mut mem_perc = 100.0 * (res as f32 / statm.size as f32); //Used memory percentage
        if mem_perc.is_nan() {
            mem_perc = 0.0;
        }
        Ok((format_number(virt), format_number(res), format_number(shr), mem_perc.to_string()))
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Invalid statm file format",
        ))
    }
}

// Get runtime of process in seconds
fn get_time(stat: &Stat) -> String {
    return ((stat.utime + stat.stime) as f32 / (procfs::ticks_per_second() as f32)).to_string();
}

// Get user from uid
fn get_user(status: &Status) -> Option<String> {
    let passwd_file = File::open("/etc/passwd").ok()?;
    let reader = BufReader::new(passwd_file);

    for line in reader.lines() {
        if let Ok(line) = line {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() >= 3 {
                if let Ok(uid) = u32::from_str(parts[2]) {
                    if uid == status.ruid {
                        return Some(parts[0].to_string());
                    }
                }
            }
        }
    }

    return None;
}

// Create vector of all processes
pub fn get_all_processes() -> Vec<ProcessInfo> {
    let mut processes: Vec<ProcessInfo> = Vec::new();
    for prc in procfs::process::all_processes().unwrap() {
        let status = prc.as_ref().unwrap().status().unwrap();
        let statm = prc.as_ref().unwrap().statm().unwrap();
        let stat = prc.as_ref().unwrap().stat().unwrap();
        let (prc_virt, prc_res, prc_shared, mem_perc) = get_mem_usage(&statm, stat.pid).unwrap();
        let username = get_user(&status).unwrap_or_else(|| "None".to_string());
        processes.push(ProcessInfo {
            pid:     stat.pid.to_string(),
            user:    username,
            virt:    prc_virt,
            res:     prc_res,
            shr:     prc_shared,
            status:  status.state.to_string(),
            cpu:     get_cpu_usage(&stat),
            mem:     mem_perc,
            time:    get_time(&stat),
            command: stat.comm
        });
        
        }
    processes.sort_by(|a, b| b.cpu.partial_cmp(&a.cpu).unwrap());
    return processes;
}

// Print process information
pub fn print_process(process: &ProcessInfo) {
    println!(
        "{:>10} {:<10} {:<8} {:<8} {:<8} {:<12} {:.5} {:.5} {:>8} {}",
        process.pid,
        process.user,
        process.virt,
        process.res,
        process.shr,
        process.status,
        process.cpu,
        process.mem,
        process.time,
        process.command)
}