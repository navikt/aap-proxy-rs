use std::io;
use std::io::{BufRead, BufReader};

pub(crate) fn is_allowed(address: &str) -> bool {
    let allowlist = read_file_to_vec("/Users/robin/rust/proxy-rs/allowlist.txt");
    match allowlist {
        Ok(allow_list) => {
            let mut allowed = false;
            allow_list.iter().for_each(|item| {
                if item.contains(&address.to_string()) {
                    tracing::info!("allowed: {} {item}", "🟢");
                    allowed = true
                } else {
                    tracing::debug!("allowed: {} {item}", "🟠")
                }
            });
            if !allowed {
                tracing::warn!("allowed: {} {address}", "🔴")
            }
            allowed
        }
        Err(_) => false,
    }
}

fn read_file_to_vec(filename: &str) -> io::Result<Vec<String>> {
    let file = std::fs::File::open(filename)?;
    let reader = BufReader::new(file);
    Ok(reader.lines().filter_map(Result::ok).collect())
}

#[cfg(test)]
mod tests {
    use crate::utils::is_allowed;

    #[test]
    fn deny() {
        let denied = "denied.co:443";
        assert!(!is_allowed(denied));
    }

    #[test]
    fn allow() {
        let allowed = "tokio.rs:443";
        assert!(is_allowed(allowed));
    }
}