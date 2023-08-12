use std::error::Error;

pub type EngineResult = Result<(), Box<dyn Error>>;