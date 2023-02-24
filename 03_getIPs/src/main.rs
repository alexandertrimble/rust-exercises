use std::process::Command;

fn main() {
    let ips = arp_scan_network();

    for ip in ips {
        println!("{}", ip);
    }
}

fn arp_scan_network() -> Vec<String> {
    let mut arp_scan = Command::new("arp");
    arp_scan.arg("-a");

    let output = arp_scan.output().expect("Failed to execute arp command");
    let output_string = String::from_utf8_lossy(&output.stdout);

    let mut ip_vec: Vec<String> = Vec::new();
    for line in output_string.lines().skip(1) {
        let ip = line.split_whitespace().next().unwrap();
        ip_vec.push(ip.to_string());
    }

    return ip_vec;
}

// fn arp_scan_network2() ->  Vec<String, String> {
//     let mut arp_scan = Command::new("arp");
//     arp_scan.arg("-a");

//     let output = arp_scan.output().expect("Failed to execute arp command");
//     let output_string = String::from_utf8_lossy(&output.stdout);

//     //put ip + address into vec
//     let mut ip_vec: Vec<String, String> = Vec::new();
//     for line in output_string.lines().skip(1) {
//         let ip = line.split_whitespace().next().unwrap();
//         let address = line.split_whitespace().next().unwrap();
//         ip_vec.push(ip.to_string(), address.to_string());
//     }

//     return ip_vec;
// }
