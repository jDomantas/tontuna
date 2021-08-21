#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Clone, Copy)]
pub struct Pos {
    pub(crate) line: u32,
    pub(crate) col: u32,
}

impl Pos {
    pub const START: Pos = Pos { line: 1, col: 1 };

    pub(crate) fn new(line: u32, col: u32) -> Pos {
        Pos { line, col }
    }

    #[deprecated]
    pub(crate) fn plus_cols(self, cols: u32) -> Pos {
        Pos {
            line: self.line,
            col: self.col + cols,
        }
    }

    pub(crate) fn plus_text(self, text: &str) -> Pos {
        #[allow(deprecated)]
        self.plus_cols(text.chars().count() as u32)
    }
}

#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy)]
pub struct Span {
    pub(crate) start: Pos,
    pub(crate) end: Pos,
}

impl Span {
    pub(crate) fn new(start: Pos, end: Pos) -> Span {
        Span { start, end }
    }

    pub(crate) fn start(self) -> Pos {
        self.start
    }

    pub(crate) fn end(self) -> Pos {
        self.end
    }
}
