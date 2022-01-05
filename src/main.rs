extern crate structopt;
mod iniparser;
mod svn;
mod subpack;
mod jenkins;
use iniparser::{get_product_first_name, locate_key, read_upgrade_ini};
use std::path::PathBuf;
use svn::get_svn_repo_info;
use subpack::get_sub_packages;
use structopt::StructOpt;
use jenkins::get_jenkins_config;
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
    let mut req_args: Vec<(&str, &str)> = vec![("hyphen", "_"), ("readme", "readme")];
    let packages = get_sub_packages(&repo_path);
    for i in 0..packages.len() {
        req_args.push(("upgrade_content", packages[i].as_str()));
    }
    req_args.push(("svn_path", svn_repo_info.repourl.as_str()));
    req_args.push(("svn_revision", svn_repo_info.revision.as_str()));
    req_args.push(("proid", main_version.as_str()));
    req_args.push(("first_name", product_first_name.as_str()));
    let url = get_jenkins_config().get_url();
    dbg!(&req_args);
    ureq::post(&url).send_form(&req_args).unwrap();
}
