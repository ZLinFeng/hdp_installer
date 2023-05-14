use std::collections::HashSet;

mod env;
mod firewalld;
mod install_jdk;
mod install_mysql;
mod install_ntp;
mod loop_info;

fn main() {
    /*  保存 Ambari Server 的 Hostname or IP */
    let ambari_server_address = loop_info::loop_ambari_server();

    println!(
        "{} Ambari Server Address is: {}",
        emojis::get_by_shortcode("white_check_mark").unwrap(),
        ambari_server_address.clone()
    );

    /*  保存 Ambari Agent 的 Hostname or IP */
    let ambari_agent_address = loop_info::loop_ambari_agents();
    for agent in ambari_agent_address.clone() {
        println!(
            "{} Ambari Agent: {}",
            emojis::get_by_shortcode("white_check_mark").unwrap(),
            agent
        );
    }

    /* 去重所有的服务器列表 */
    let mut address_list = ambari_agent_address.clone();
    address_list.push(ambari_server_address.clone());
    let set: HashSet<String> = address_list.into_iter().collect();
    address_list = set.into_iter().collect();

    /* 1. 关闭所有系统的防火墙 */
    firewalld::stop_firewalld(&address_list);

    /* 2. 安装ntp服务 */
    install_ntp::install_online(&address_list, &ambari_server_address);

    /* 3. 安装jdk */
    install_jdk::install_jdk(&address_list);

    /* 4. 在Ambari Server上安装 MySQL  */
}
