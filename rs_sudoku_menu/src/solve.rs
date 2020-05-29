use brute_solver_lib::{
    file_reader::FileReader,
    puzzle_state::PuzzleState,
    brute_solver::BruteSolver,
};

use crate::args_processor::ArgsProcessor;

pub struct Solve;

impl Solve {

    pub fn start(args: ArgsProcessor) {
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
}