use argh::FromArgs;

use rustling_dark::exercise::{self, Exercise, Status};
use rustling_dark::watcher::Watcher;

#[derive(FromArgs, PartialEq, Debug)]
/// Rustlings is a collection of small exercises to get you used to writing and reading Rust code
struct Args {
    /// show outputs from the test exercises
    #[argh(switch)]
    nocapture: bool,
    /// show the executable version
    #[argh(switch, short = 'v')]
    version: bool,
    #[argh(subcommand)]
    nested: Option<Subcommands>,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum Subcommands {
    // Verify(VerifyArgs),
    Watch(WatchArgs),
    // Run(RunArgs),
    // Reset(ResetArgs),
    // Hint(HintArgs),
    // List(ListArgs),
    // Lsp(LspArgs),
}

const VERSION: &str = "1";

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "watch")]
/// Reruns `verify` when files were edited
struct WatchArgs {}

fn main() {
    let args: Args = argh::from_env();
    println!("Args received {:?}", args);
    // println!("Commands {:?}", argh::SubCommand);
    if args.version {
        println!("Version is: {}", VERSION)
    }

    let exercises = exercise::get_exercises();
    println!("Exercises are {:?}", exercises);
    match args.nested {
        None => println!("No subcommand was passed"),
        Some(Subcommands::Watch(_)) => watch(exercises),
    }
}


fn watch(exercises: Vec<Exercise>) {
    println!("Watch mode was activated");
    for mut ex in exercises {
        let _ = match ex.check_status() {
            Ok(Status::Done) => {
                println!("Exercise {} was done!", ex.name)
            },
            Ok(Status::Pending) => {
                println!("Exercise {} is pending!", ex.name);
                println!("Las time modification {:?}",ex.get_last_mod());
                let mut watcher  = Watcher::new(ex);
                watcher.watch()

            },
            Err(e) => {
                println!("Parsing {} is threw the following Error:\n {}", ex.name, e)
            }
        };
        
        
    }
}
