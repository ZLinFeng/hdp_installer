use std::{
    net::{IpAddr, Ipv4Addr, Ipv6Addr, ToSocketAddrs},
    str::FromStr,
};

use ping;

use emojis;

/**
 * 检查地址的连通性
 */
pub fn check_connection(address: &String) -> bool {
    let address_str = address.as_str().replace("\n", "");
    let address_str = address_str.as_str();

    let ipv4_addr = Ipv4Addr::from_str(address_str);
    match ipv4_addr {
        Ok(ip) => {
            println!(
                "{} Connect IPV4 Address: {}",
                emojis::get_by_shortcode("crab").unwrap(),
                ip
            );
            return connect_ipv4(ip);
        }
        Err(_) => {}
    }
    let ipv6_addr = Ipv6Addr::from_str(address_str);
    match ipv6_addr {
        Ok(_) => {
            println!(
                "{} IPV6 is not support.",
                emojis::get_by_shortcode("prohibited").unwrap()
            );
        }
        Err(_) => {}
    }

    if let Some(ip_addr) = resolve_hostname(address_str) {
        let ipv4 = match ip_addr {
            IpAddr::V4(v4) => {
                println!(
                    "{} Convert hostname[{}] to IP[{}]",
                    emojis::get_by_shortcode("white_check_mark").unwrap(),
                    address_str,
                    v4.to_string()
                );
                Some(v4)
            }
            _ => None,
        };
        if let Some(v4) = ipv4 {
            return connect_ipv4(v4);
        } else {
            return false;
        }
    } else {
        return false;
    }
}

/**
 * 检查 ipv4 连通性
 */
fn connect_ipv4(ip: Ipv4Addr) -> bool {
    let response = ping::ping(IpAddr::from(ip), None, None, None, None, None);
    match response {
        Ok(()) => {
            println!(
                "{} Connect success.",
                emojis::get_by_shortcode("white_check_mark").unwrap()
            );
            return true;
        }
        Err(_) => {
            println!(
                "{} {} is not available.",
                emojis::get_by_shortcode("no_entry_sign").unwrap(),
                ip.to_string()
            );
            return false;
        }
    }
}

/**
 * 解析域名
 */
fn resolve_hostname(hostname: &str) -> Option<IpAddr> {
    let hostname_socket = format!("{}:22", hostname);
    let addrs_iter = match hostname_socket.to_socket_addrs() {
        Ok(iter) => iter,
        Err(e) => {
            println!(
                "{} {}: {}",
                emojis::get_by_shortcode("no_entry_sign").unwrap(),
                hostname,
                e
            );
            return None;
        }
    };

    for addr in addrs_iter {
        let ip_addr = addr.ip();
        return Some(ip_addr);
    }

    None
}

#[test]
fn print_emoji_short_code() {
    let a = emojis::get("🚫").unwrap().shortcode();
    if let Some(b) = a {
        println!("{}", b);
    }
}
