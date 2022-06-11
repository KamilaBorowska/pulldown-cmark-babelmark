use cargo_lock::Lockfile;
use itertools::Itertools;

fn main() {
    println!("cargo:rerun-if-changed=Cargo.lock");
    let lockfile = Lockfile::load("Cargo.lock").expect("a valid lockfile");
    let version = &lockfile
        .packages
        .iter()
        .filter(|p| p.name.as_str() == "pulldown-cmark")
        .exactly_one()
        .expect("a dependency called pulldown-cmark")
        .version;
    println!("cargo:rustc-env=PULLDOWN_CMARK_VERSION={version}");
}
