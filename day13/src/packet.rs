pub const LEFTBRACE: char = '[';
pub const RIGHTBRACE: char = ']';

pub mod parse {

    #[derive(PartialEq, Copy, Clone, Debug)]
    pub enum Correct {
        LeftSideRanOutItems,
        LeftSideIsSmaller,
    }

    #[derive(PartialEq, Copy, Clone, Debug)]
    pub enum Incorrect {
        RightSideRanOutItems,
        RightSideIsSmaller,
    }

    #[derive(PartialEq, Copy, Clone, Debug)]
    pub enum Order {
        Correct(Correct),
        Incorrect(Incorrect),
    }
}
