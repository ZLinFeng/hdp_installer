use crate::env::transfer_file_without_password;
use ssh2::Session;
use std::{fs, io::Write, path::Path};

static TMP_JDK_PATH: &str = "/tmp/jdk1.8.0_112.tar.gz";

pub fn install_jdk(address_list: &Vec<String>) {
    let jdk_file = include_bytes!("../bin/jdk1.8.0_112.tar.gz");
    if let Ok(metadata) = fs::metadata(TMP_JDK_PATH) {
        if metadata.is_file() {
            fs::remove_file(TMP_JDK_PATH).unwrap();
        }
    }
    let mut tmp_jdk_file = fs::File::create(TMP_JDK_PATH).unwrap();
    tmp_jdk_file.write_all(jdk_file).unwrap();

    for addr in address_list {
        inner_install_jdk(addr);
    }
}

fn inner_install_jdk(addr: &String) {
    let tcp = std::net::TcpStream::connect(format!("{}:22", addr)).unwrap();
    let mut session = Session::new().unwrap();
    session.set_tcp_stream(tcp);
    session.handshake().unwrap();

    let mut channel = session.channel_session().unwrap();
    channel
        .exec(format!("rm -rf {}", TMP_JDK_PATH).as_str())
        .unwrap();

    let local_path = Path::new(TMP_JDK_PATH);
    let remote_path = Path::new(TMP_JDK_PATH);
    transfer_file_without_password(&local_path, &remote_path, addr);

    channel.exec("rpm --nodeps -e `rpm -qa|grep jdk`").unwrap();
    channel.exec("mkdir -p /opt/java/jdk1.8.0_112").unwrap();
    channel.exec("rm -rf /opt/java/jdk1.8.0_112").unwrap();
    channel
        .exec("tar -zxf jdk1.8.0_112.tar.gz -C /opt/java")
        .unwrap();
    channel
        .exec("echo \"export JAVA_HOME=/opt/java/jdk1.8.0_112/\" >> /etc/profile")
        .unwrap();
    channel
        .exec("echo \"export JRE_HOME=/opt/java/jdk1.8.0_112/jre\" >> /etc/profile")
        .unwrap();
    channel
        .exec("echo \"export PATH=$JAVA_HOME/bin:$JRE_HOME/bin:$PATH\" >> /etc/profile")
        .unwrap();
    channel.exec("source /etc/profile").unwrap();
    channel
        .exec(format!("rm -rf {}", TMP_JDK_PATH).as_str())
        .unwrap();
    channel.close().unwrap();
    session.disconnect(None, "Bye", None).unwrap();
}
