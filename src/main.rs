use clap::{arg, App, AppSettings};
use log::{error, info};

mod core;

use self::core::passwordfile::PasswordFile;

fn main() {
    env_logger::init();

    let app = clap::App::new("authenticator")
        .bin_name("authenticator")
        .setting(clap::AppSettings::SubcommandRequired)
        .subcommand(
            App::new("hash")
                .about("return the SHA-256 hash of the supplied text")
                .arg(arg!(<TEXT> "the text to hash"))
                .setting(AppSettings::ArgRequiredElseHelp),
        )
        .subcommand(App::new("entries").about("list entries in the password file"))
        .subcommand(
            App::new("adduser")
                .about("adds user in the password file")
                .arg(arg!(<USER> "the user name"))
                .arg(arg!(<PWD> "the user password"))
                .setting(AppSettings::ArgRequiredElseHelp),
        )
        .subcommand(
            App::new("removeuser")
                .about("removes a user from the password file")
                .arg(arg!(<USER> "the user name"))
                .setting(AppSettings::ArgRequiredElseHelp),
        )
        .get_matches();

    const PASSWORD_PATH: &str = "pwd.json";
    let path = std::path::Path::new(PASSWORD_PATH);

    let mut pw = match PasswordFile::load(path) {
        Ok(pwd) => {
            info!("loaded password file {}", PASSWORD_PATH);
            pwd
        }
        Err(err) => {
            error!("{}", err);
            info!("creating new password file");
            PasswordFile::new()
        }
    };

    match app.subcommand() {
        Some(("hash", sub_matches)) => {
            let text = sub_matches.value_of("TEXT").unwrap();
            println!("SHA-256 of \"{}\": {}", text, core::hash(text));
        }
        Some(("entries", _)) => {
            show_entries(&pw);
        }
        Some(("adduser", sub_matches)) => {
            match (sub_matches.value_of("USER"), sub_matches.value_of("PWD")) {
                (Some(user), Some(pwd)) => add_user(&mut pw, user, pwd),
                _ => println!("usage: adduser <USER> <PWD>"),
            }
        }
        Some(("removeuser", sub_matches)) => match sub_matches.value_of("USER") {
            Some(user) => remove_user(&mut pw, user),
            _ => println!("usage: removeuser <USER>"),
        },
        _ => unreachable!(),
    }

    match pw.save(path) {
        Ok(()) => info!("saved password file to {}", PASSWORD_PATH),
        Err(err) => error!("{}", err),
    }
}

fn show_entries(file: &PasswordFile) {
    let users = file.enumerate();
    if users.is_empty() {
        println!("the password file has no users.");
    } else {
        println!("the file contains the following users:");
        let mut index = 1;
        for (user, hash) in users {
            println!("#{: >2} {: >15} (password hash: {})", index, user, hash);
            index += 1;
        }
    }
}

fn remove_user(file: &mut PasswordFile, user: &str) {
    file.remove_user(user);
    println!("removed user {}", user);
}

fn add_user(file: &mut PasswordFile, user: &str, password: &str) {
    let hash = core::hash(password);
    println!("adding user {} (password hash: {})", user, hash);
    file.add_user(user, hash.as_str());
}
