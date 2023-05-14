use std::{
    fs,
    io::{Read, Write},
    path::Path,
};

use crate::env::transfer_file_without_password;

use ssh2::Session;

static ONLINE_CMD: &str = "yum install -y ntp";
static RESTRART: &str = "systemctl restart ntpd";
static ENABLE: &str = "systemctl enable ntpd";

pub fn install_online(address_list: &Vec<String>, server_address: &String) {
    for addr in address_list {
        let is_server = addr.as_str() == server_address.as_str();
        inner_install_online(&addr, server_address, is_server);
    }
}

/**
 * 在线安装 ntp
 */
fn inner_install_online(addr: &String, server: &String, is_server: bool) {
    let tcp = std::net::TcpStream::connect(format!("{}:22", addr)).unwrap();
    let mut session = Session::new().unwrap();
    session.set_tcp_stream(tcp);
    session.handshake().unwrap();
    //session.userauth_password("root", "").unwrap();
    let mut channel = session.channel_session().unwrap();
    channel.exec(ONLINE_CMD).unwrap();
    println!(
        "{} Install ntp on {}.",
        emojis::get_by_shortcode("white_check_mark").unwrap(),
        &addr
    );
    let mut s = String::new();
    channel.read_to_string(&mut s).unwrap();
    println!("{}", s);

    let tmp_ntp = "/tmp/ntp.conf";
    if let Ok(metadata) = fs::metadata(tmp_ntp) {
        if metadata.is_file() {
            fs::remove_file(tmp_ntp).unwrap();
        }
    }
    let mut ntp_config = String::from("driftfile /var/lib/ntp/drift\nrestrict default nomodify\n");
    if is_server {
        ntp_config += "server 127.127.1.0\n";
        ntp_config += "fudge 127.127.1.0 stratum 10\n";
    } else {
        ntp_config += format!("server {}\n", server).as_str();
        ntp_config += format!("fudge {} stratum 10\n", server).as_str();
    }
    let mut config_file = fs::File::create(tmp_ntp).unwrap();
    config_file.write_all(ntp_config.as_bytes()).unwrap();

    transfer_file_without_password(Path::new(tmp_ntp), Path::new("/etc/ntp.conf"), addr);

    channel.exec(RESTRART).unwrap();
    println!(
        "{} Restart ntp service on {}.",
        emojis::get_by_shortcode("white_check_mark").unwrap(),
        &addr
    );
    let mut s = String::new();
    channel.read_to_string(&mut s).unwrap();
    println!("{}", s);

    channel.exec(ENABLE).unwrap();
    println!(
        "{} Enable ntp service on {}.",
        emojis::get_by_shortcode("white_check_mark").unwrap(),
        &addr
    );
    let mut s = String::new();
    channel.read_to_string(&mut s).unwrap();
    println!("{}", s);

    channel.close().unwrap();
    session.disconnect(None, "", None).unwrap();
}

pub fn install_offline(address_list: &Vec<String>) {
    for addr in address_list {
        inner_install_offline(&addr);
    }
}

/**
 * 离线安装 ntp
 */
fn inner_install_offline(addr: &String) {}
