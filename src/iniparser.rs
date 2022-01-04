use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};

pub fn read_upgrade_ini(repo_path: &PathBuf) -> HashMap<String, String> {
    let mut ini_options = HashMap::new();
    if let Ok(lines) = read_lines(repo_path.join("UpgradeList.ini")) {
        for line in lines {
            if let Ok(kvpair) = line {
                kvpair.find("=").and_then(|atindex| {
                    let (key, value) = kvpair.split_at(atindex);
                    if ini_options.contains_key(key) {
                        panic!("Duplicated key {}", key);
                    }
                    ini_options.insert(key.to_string(), value[1..].trim().to_string())
                });
            }
        }
    };
    ini_options
}

pub fn get_product_first_name(product: &str) -> String {
    let product_names: HashMap<&str, &str> = HashMap::from([
        ("LAS", "LAS:深信服LAS"),
        ("BVT", "BVT:深信服BVT"),
        ("BDSEC", "001:BDSEC"),
        ("SAS", "002:SAS"),
        ("CSV", "003:CSV"),
        ("BDLOG", "004:BDLOG"),
        ("NFA", "005:iNFA"),
    ]);
    product_names
        .get(&product.to_ascii_uppercase().as_str())
        .expect("Iilleagal product name.")
        .to_string()
}

pub fn locate_key<'a>(
    upgrade_info: &'a HashMap<String, String>,
    keyword: &str,
) -> Option<&'a String> {
    for (key, val) in upgrade_info.iter() {
        if key.to_ascii_lowercase().contains(keyword) && val.chars().count() > 0 {
            return upgrade_info.get(key);
        }
    }
    None
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
