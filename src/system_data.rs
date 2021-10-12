use sysinfo::{NetworkExt, ProcessExt, System, SystemExt};

pub fn print_all_system_data() {
    // Please note that we use "new_all" to ensure that all list of
// components, network interfaces, disks and users are already
// filled!
    let mut sys = System::new_all();

// First we update all information of our `System` struct.
    sys.refresh_all();

// We display all disks' information:
    println!("=> disks:");
    for disk in sys.disks() {
        println!("{:?}", disk);
    }

// Network interfaces name, data received and data transmitted:
    println!("=> networks:");
    for (interface_name, data) in sys.networks() {
        println!("{}: {}/{} B", interface_name, data.received(), data.transmitted());
    }

// Components temperature:
    println!("=> components:");
    for component in sys.components() {
        println!("{:?}", component);
    }

    println!("=> system:");
// RAM and swap information:
    println!("total memory: {} KB", sys.total_memory());
    println!("used memory : {} KB", sys.used_memory());
    println!("total swap  : {} KB", sys.total_swap());
    println!("used swap   : {} KB", sys.used_swap());

// Display system information:
    println!("System name:             {:?}", sys.name());
    println!("System kernel version:   {:?}", sys.kernel_version());
    println!("System OS version:       {:?}", sys.os_version());
    println!("System host name:        {:?}", sys.host_name());

// Number of processors:
    println!("NB processors: {}", sys.processors().len());

// // Display processes ID, name na disk usage:
//     for (pid, process) in sys.processes() {
//         if process.name().eq("vivaldi-bin") {
//             println!("[{}] {} {:?}", pid, process.name(), process.disk_usage());
//         }
//     }
    print_processes(&sys);
}

pub fn print_processes(system: &System){
    for (pid, process) in system.processes() {
        println!("[{}] {} {:?}", pid, process.name(), process.disk_usage())
    }
}