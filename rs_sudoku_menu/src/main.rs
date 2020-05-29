mod menu;
mod build;
mod solve;
mod args_processor;

extern crate puzzle_maker_lib;
extern crate brute_solver_lib;

use build::Build;
use solve::Solve;
use args_processor::ArgsProcessor;

use menu::Menu;

fn main() {

    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 {
        println!("{}", Menu::display());
        std::process::exit(0);
    }

    let input_args = ArgsProcessor::parse_args(args);

    match input_args.action {
        SWITCH_BUILD => {
            Build::start(input_args);
        },
        SWITCH_SOLVE => {
            Solve::start(input_args);
        }
        _ => (),
    }
}