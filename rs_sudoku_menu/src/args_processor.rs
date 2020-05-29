#[derive(Default)]
pub struct ArgsProcessor {
    pub action: &'static str,
    pub min_clues: u32,
    pub max_clues: u32,
}

impl ArgsProcessor {

    pub const SWITCH_BUILD: &'static str = "--build";
    pub const SWITCH_SOLVE: &'static str = "--solve";
    pub const SWITCH_MIN_CLUES: &'static str = "--min-clues";
    pub const SWITCH_MAX_CLUES: &'static str = "--max-clues";

    pub fn parse_args(args: Vec<String>) -> ArgsProcessor {
        let mut args_processor = ArgsProcessor::default();

        for x in 0..args.len() {
            match args[x].as_str() {
                ArgsProcessor::SWITCH_BUILD => args_processor.action = ArgsProcessor::SWITCH_BUILD,
                ArgsProcessor::SWITCH_SOLVE => args_processor.action = ArgsProcessor::SWITCH_SOLVE,
                ArgsProcessor::SWITCH_MAX_CLUES => args_processor.max_clues = args[x + 1]
                    .parse::<u32>().expect("Error parsing number for --max-clues value"),
                ArgsProcessor::SWITCH_MIN_CLUES => args_processor.min_clues = args[x + 1]
                    .parse::<u32>().expect("Error parsing number for --min-clues value"),
                _ => (),
            }
        }

        args_processor
    }
}