use std::io::{self, Write};

use crate::env::check_connection;

/**
 * 等待输入Ambari Server的hostname
 */
pub fn loop_ambari_server() -> String {
    loop {
        let question_mark = emojis::get_by_shortcode("question").unwrap();
        println!(
            "{} Please enter hostname or IP of Ambari Server: (Example: hdp1 or 192.168.1.1)",
            question_mark
        );
        let computer_mark = emojis::get_by_shortcode("computer").unwrap();
        print!("{} > ", computer_mark);
        io::stdout().flush().unwrap();

        let mut server_address = String::new();
        io::stdin().read_line(&mut server_address).unwrap();
        println!(
            "{} Check connection.......",
            emojis::get_by_shortcode("crab").unwrap()
        );
        if check_connection(&server_address) {
            return server_address;
        } else {
            println!(
                "{} Please enter a available address.",
                emojis::get_by_shortcode("no_entry_sign").unwrap()
            );
        }
    }
}

/**
 * 等待输入Ambari Agent的hostname
 */

pub fn loop_ambari_agents() -> Vec<String> {
    loop {
        let question_mark = emojis::get_by_shortcode("question").unwrap();
        println!(
            "{} Please enter hostname or IP of Ambari Agents: (Example: hdp1,hdp2 or hdp1,192.168.1.1 or 192.168.1.1,192.168.1.2)",
            question_mark
        );
        let computer_mark = emojis::get_by_shortcode("computer").unwrap();
        print!("{} > ", computer_mark);
        io::stdout().flush().unwrap();

        let mut agents_address_str = String::new();
        io::stdin().read_line(&mut agents_address_str).unwrap();
        agents_address_str = agents_address_str.replace("\n", "");
        println!(
            "{} Check connection.......",
            emojis::get_by_shortcode("crab").unwrap()
        );
        let mut agent_list: Vec<String> = Vec::new();
        let agents = agents_address_str.split(",");
        for agent_str in agents {
            let agent = String::from(agent_str);
            if check_connection(&agent) {
                agent_list.push(agent);
            }
        }
        if agent_list.len() > 0 {
            return agent_list;
        } else {
            println!(
                "{} There is no available agent.",
                emojis::get_by_shortcode("no_entry_sign").unwrap()
            );
        }
    }
}
