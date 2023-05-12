use std::collections::HashSet;

mod env;
mod firewalld;
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
    /* 1. 关闭所有系统的防火墙 */
    let mut address_list = ambari_agent_address.clone();
    address_list.push(ambari_server_address.clone());
    let set: HashSet<String> = address_list.into_iter().collect();
    address_list = set.into_iter().collect();
    firewalld::stop_firewalld(address_list);
}
