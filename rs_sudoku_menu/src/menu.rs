pub struct Menu;

impl Menu {


    pub fn display() -> String {
        let menu: String = format!("
--build                 -- build a board (default action)
--min-clues [num]       -- default is 45
--max-clues [num]       -- default is 39 (less than 39 may not be solvable)
--solve [board file]    -- Solve a board");
        
        menu
    }
}