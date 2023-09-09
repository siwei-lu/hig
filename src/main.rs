use clap::Command;
use hig::cmd;

fn cli() -> Command {
    Command::new("hig")
        .version("0.1.0")
        .about("A CLI for Better Git Experience")
        .author("Siwei LU <me@siwei.lu>")
        .arg_required_else_help(true)
        .subcommand(cmd::feature::new())
        .subcommand(cmd::rm::new())
        .subcommand(cmd::reset::new())
        .subcommand(cmd::config::new())
}

fn main() {
    let matched = cli().get_matches();

    let result = match matched.subcommand() {
        Some((cmd::feature::NAME, feature)) => {
            let name = feature.get_one::<String>("name").unwrap();
            cmd::feature::run(name)
        }
        Some((cmd::rm::NAME, _)) => cmd::rm::run(),
        Some((cmd::reset::NAME, _)) => cmd::reset::run(),
        Some((cmd::config::NAME, config)) => {
            let key = config.get_one::<String>("key").unwrap();
            let value = config.get_one::<String>("value").unwrap();
            cmd::config::run(key, value)
        }
        _ => Ok(()),
    };

    if let Err(e) = result {
        eprintln!("{}", e);
    }
}
