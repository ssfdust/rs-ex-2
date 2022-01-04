extern crate structopt;
mod iniparser;
mod svn;
mod subpack;
use iniparser::{get_product_first_name, locate_key, read_upgrade_ini};
use std::path::PathBuf;
use svn::get_svn_repo_info;
use subpack::get_sub_packages;
use structopt::StructOpt;
use std::fs;

#[derive(StructOpt)]
struct ArgumentParser {
    #[structopt(default_value = ".", parse(from_os_str))]
    repopath: PathBuf,
}

fn main() {
    let parser = ArgumentParser::from_args();
    let repo_path = fs::canonicalize(parser.repopath).unwrap();
    let svn_repo_info = get_svn_repo_info(&repo_path);
    let upgrade_info = read_upgrade_ini(&repo_path);
    let main_version = locate_key(&upgrade_info, "version").expect("Cannot get the main version");
    let product_first_name = get_product_first_name(
        locate_key(&upgrade_info, "product").expect("Cannot get product info"),
    );
    for pack in get_sub_packages(&repo_path) {
        println!("package: {}", pack);
    }
    println!("{}", svn_repo_info);
    println!("Period: {}", main_version);
    println!("first_name: {}", product_first_name);
}
