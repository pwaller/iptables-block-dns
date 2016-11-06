use std::process::{Command, Stdio};

extern crate clap;
use clap::{App, ArgGroup};

extern crate rustc_serialize;
use rustc_serialize::hex::ToHex;

fn main() {
    let app = App::new("iptables-block-dns")
        .version("0.0.0")
        .about("\nBlock sites by blocking outgoing DNS packets with iptables")
        .args_from_usage(
            "-a, --add 'Insert rule'
             -d, --del 'Delete rule'
             <hostname> 'Host'")
        .group(ArgGroup::with_name("action")
             .args(&["add", "del"])
             .required(true));

    let m = app.get_matches();
    let hostname = m.value_of("hostname").unwrap();

    let action = if m.is_present("add") { "--append" } else { "--delete" };

    // Transform the input into the form in which it will appear in DNS packets.
    // e.g: foo.bar => 0x03{hex for "foo"}0x03{hex for "bar"}.
    let hexed = hostname
        .split(".")
        .map(|c| {
            // e.g, 0x03{hex for c}
            format!("{:02x}{}", c.len(), c.as_bytes().to_hex())
        })
        .collect::<Vec<_>>()
        .concat();

    let target_string = format!("|{}|", hexed);

    let status =
        // Command::new("echo")
        // .arg("/sbin/iptables")
        Command::new("/sbin/iptables")
        .stdout(Stdio::inherit())
        .arg("--wait").arg("2")
        .arg(action).arg("OUTPUT") // chain
        .arg("--protocol").arg("udp")
        .arg("--dport").arg("domain")
        .arg("--match").arg("string")
        .arg("--hex-string").arg(target_string)
        .arg("--algo").arg("bm")
        .arg("--jump").arg("DROP")
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    if !status.success() {
        println!("iptables failed with exit status: {:?}", status.code().unwrap());
        std::process::exit(1);
    }
}
