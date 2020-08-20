use clap::{load_yaml, App};

fn main() {
    let cli_yaml = load_yaml!("cli.yaml");
    let matches = App::from(cli_yaml).get_matches();
}
