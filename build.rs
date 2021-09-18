// in build.rs
use clap::{crate_version, load_yaml, App, AppSettings};
use clap_generate::{
    generate_to,
    generators::{Bash, Zsh},
};
use std::env;

fn main() {
    let yaml = load_yaml!("src/cli.yaml");
    let mut app = App::from(yaml)
        .version(crate_version!())
        .setting(AppSettings::SubcommandRequiredElseHelp);

    app.set_bin_name("rustybam");
    //let out_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("completions/");
    let out_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    generate_to::<Bash, _, _>(&mut app, "rustybam", &out_dir)
        .expect("Failed to generate bash completions");
    generate_to::<Zsh, _, _>(&mut app, "rustybam", &out_dir)
        .expect("Failed to generate zsh completions");
}
