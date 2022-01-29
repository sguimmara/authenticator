use clap::{arg, App, AppSettings};

mod core;

fn main() {
    let app = clap::App::new("authenticator")
        .bin_name("authenticator")
        .setting(clap::AppSettings::SubcommandRequired)
        .subcommand(
            App::new("hash")
                .about("return the SHA-256 hash of the supplied text")
                .arg(arg!(<TEXT> "the text to"))
                .setting(AppSettings::ArgRequiredElseHelp),
        )
        .get_matches();

    match app.subcommand() {
        Some(("hash", sub_matches)) => {
            let text = sub_matches.value_of("TEXT").unwrap();
            println!("SHA-256 of \"{}\": {}", text, core::hash(text));
        }
        _ => unreachable!(),
    }
}
