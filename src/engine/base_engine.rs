use super::constants::EngineResult;

#[derive(Clone, Debug)]
pub struct VoidEngine {
    pub quit: bool
}

impl VoidEngine {
    pub fn new() -> Self {
        Self {
            quit: false
        }
    }

    pub async fn run(&mut self) -> EngineResult {
        self.run_forever().await;
        Ok(())
    }

    async fn run_forever(&mut self) {
        while !self.quit {
            //TODO: Accept UCI command here.
        }

        self.quit_fn();
    }

    fn quit_fn(&mut self) {
        self.quit = true;
        
    }
}