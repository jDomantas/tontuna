pub(crate) struct Compilation {
    source: String,
}

impl Compilation {
    pub(crate) fn from_source(source: String) -> Compilation {
        Compilation { source }
    }

    pub(crate) fn source(&self) -> &str {
        &self.source
    }
}
