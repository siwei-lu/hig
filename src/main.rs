use clap::Command;
use hig::cmd;

fn cli() -> Command {
    Command::new("hig")
        .version("0.1.0")
        .about("A CLI for Better Git")
        .author("Siwei Lu <me@siwei.lu>")
        .arg_required_else_help(true)
        .subcommand(cmd::feature::new())
}

fn main() {
    let matched = cli().get_matches();

    match matched.subcommand() {
        Some((cmd::feature::NAME, feature)) => {
            let name = feature.get_one::<String>("name").unwrap();
            cmd::feature::run(name);
        }
        _ => {
            println!("No command matched");
        }
    }
}
