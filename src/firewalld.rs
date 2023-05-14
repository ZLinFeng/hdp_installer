use std::io::Read;

use ssh2::Session;

static STOP_FIREWALLD_CMD: &str = "systemctl stop firewalld";
static DIS_FIREWALLD_CMD: &str = "systemctl disable firewalld";

pub fn stop_firewalld(address_list: &Vec<String>) {
    for address in address_list {
        inner_stop_firewalld(address);
    }
}

fn inner_stop_firewalld(addr: &String) {
    let tcp = std::net::TcpStream::connect(format!("{}:22", addr)).unwrap();
    let mut session = Session::new().unwrap();
    session.set_tcp_stream(tcp);
    session.handshake().unwrap();

    let mut channel = session.channel_session().unwrap();
    channel.exec(STOP_FIREWALLD_CMD).unwrap();
    println!(
        "{} Stop firewalld on {}.",
        emojis::get_by_shortcode("white_check_mark").unwrap(),
        &addr
    );
    let mut s = String::new();
    channel.read_to_string(&mut s).unwrap();
    println!("{}", s);

    channel.exec(DIS_FIREWALLD_CMD).unwrap();
    println!(
        "{} Disable firewalld on {}.",
        emojis::get_by_shortcode("white_check_mark").unwrap(),
        &addr
    );
    let mut s = String::new();
    channel.read_to_string(&mut s).unwrap();
    println!("{}", s);
    channel.close().unwrap();
    session.disconnect(None, "", None).unwrap();
}

#[test]
fn test_stop() {
    let address_list = vec![String::from("10.11.203.172")];

    stop_firewalld(&address_list);
}
