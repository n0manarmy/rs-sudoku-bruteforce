extern crate puzzle_maker_lib;
extern crate brute_solver_lib;

mod menu;

use puzzle_maker_lib::{
    board_builder::BoardBuilder,
    puzzle_maker::PuzzleMaker,
};
use brute_solver_lib::{
    file_reader::FileReader,
    puzzle_state::PuzzleState,
    brute_solver::BruteSolver,
};




fn main() {

    const SWITCH_BUILD: &str = "--build";
    const SWITCH_SOLVE: &str = "--solve";

    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 {
        println!(
            "--build -- build a board\n--solve [board file] -- Solve a board");
        std::process::exit(0);
    }

    match args[1].parse::<String>().unwrap().as_ref() {

        SWITCH_BUILD => {
            let mut puzzle_board = BoardBuilder::build_brute();
            BoardBuilder::write_to_file(&puzzle_board, ".answer");

            puzzle_board = PuzzleMaker::apply_pattern(puzzle_board, PuzzleMaker::PATTERN_STAR.to_vec());
            puzzle_board = PuzzleMaker::apply_pattern(puzzle_board, PuzzleMaker::PATTERN_EAST_TO_WEST.to_vec());

            BoardBuilder::write_to_file(&puzzle_board, ".puzzle");
            puzzle_board.print_stats();


        },
        SWITCH_SOLVE => {
            let file = args[2].parse::<String>().unwrap();    
            let file = FileReader::read(&file);
            let input = serde_json::from_str(&file).unwrap();
            match PuzzleState::with_values(input) {
            Ok(mut result) => {
                let grid_results = BruteSolver::solve(&mut result);
                BruteSolver::display_output(grid_results);       
            },
            Err(why) => panic!(why.to_string()),
            }   
        }
        _ => (),
    }
}