pub mod types {
    #[derive(Debug)]
    pub enum Line {
        CdCommand { location: String },
        LsCommand,
        DirOutput { name: String },
        FileOutput { size: usize, name: String },
    }
}
