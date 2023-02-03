struct Pipeline {
    context: String,
    stages: Vec<Stage>,
}

impl Pipeline {
    fn new() -> Self {
        Self {
            context: String::new(),
            stages: Vec::new(),
        }
    }

    fn add_stage(&mut self, stage: Stage) {
        self.stages.push(stage);
    }
}
