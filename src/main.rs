use clap::{load_yaml, App};
use cli_toolbox::{debugln, reportln};
use verbosity::Verbosity;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    // Get command line arguments
    let source_path = matches.value_of("source").unwrap();
    let target_path = matches.value_of("target").unwrap();
    match matches.occurrences_of("verbose") {
        0 => Verbosity::Quite.set_as_global(),
        1 => Verbosity::Terse.set_as_global(),
        2 | _ => Verbosity::Verbose.set_as_global(),
    };

    reportln!(
        @terse "Synchronizing…";
        @verbose "Sync from '{}' to '{}' with verbosity'", source_path, target_path;
    )
}
