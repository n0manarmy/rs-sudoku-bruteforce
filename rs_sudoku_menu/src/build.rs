use puzzle_maker_lib::{
    board_builder::BoardBuilder,
    puzzle_maker::PuzzleMaker,
};

use crate::args_processor::ArgsProcessor;

pub struct Build;

impl Build {

    pub fn start(args: ArgsProcessor) {
        let reset_threshold = 500000; //how many runs before we reset the board cause of constant failures
        let build_resets_stop = usize::MAX; //how many board resets we run before we stop execution
        let mut puzzle_board = BoardBuilder::build_brute(reset_threshold, build_resets_stop);

        BoardBuilder::write_to_file(&puzzle_board, "_answer");
        puzzle_board = PuzzleMaker::apply_patterns(puzzle_board, 45);
        BoardBuilder::write_to_file(&puzzle_board, "_puzzle");
        puzzle_board.print_board();
    }
}