use ssh2::Session;

use crate::env::transfer_file_without_password;
use std::{fs, io::Write, path::Path};

static TMP_MYSQL_PATH: &str = "/tmp/mysql.tar.gz";

pub fn install_mysql(address: &String) {
    let mysql_file = include_bytes!("../bin/mysql.tar.gz");
    if let Ok(metadata) = fs::metadata(TMP_MYSQL_PATH) {
        if metadata.is_file() {
            fs::remove_file(TMP_MYSQL_PATH).unwrap();
        }
    }
    let mut tmp_mysql_file = fs::File::create(TMP_MYSQL_PATH).unwrap();
    tmp_mysql_file.write_all(mysql_file).unwrap();

    let tcp = std::net::TcpStream::connect(format!("{}:22", address)).unwrap();
    let mut session = Session::new().unwrap();
    session.set_tcp_stream(tcp);
    session.handshake().unwrap();

    let mut channel = session.channel_session().unwrap();
    channel
        .exec(format!("rm -rf {}", TMP_MYSQL_PATH).as_str())
        .unwrap();

    let local_path = Path::new(TMP_MYSQL_PATH);
    let remote_path = Path::new(TMP_MYSQL_PATH);
    transfer_file_without_password(&local_path, &remote_path, address);

    channel.exec("rm -rf /tmp/mysql").unwrap();
    channel
        .exec("rpm --nodeps -e `rpm -qa|grep mariadb`")
        .unwrap();
    channel
        .exec(format!("tar -zxf {} -C /tmp/", TMP_MYSQL_PATH).as_str())
        .unwrap();
    channel
        .exec("yum local install /tmp/mysql/mysql-community-common-5.7.27-1.el7.x86_64.rpm")
        .unwrap();
    channel
        .exec("yum local install /tmp/mysql/mysql-community-libs-5.7.27-1.el7.x86_64.rpm")
        .unwrap();
    channel
        .exec("yum local install /tmp/mysql/mysql-community-client-5.7.27-1.el7.x86_64.rpm")
        .unwrap();
    channel
        .exec("yum local install /tmp/mysql/mysql-community-server-5.7.27-1.el7.x86_64.rpm")
        .unwrap();
}
