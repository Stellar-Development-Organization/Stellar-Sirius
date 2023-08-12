#![allow(non_snake_case)]

use crate::engine::{
    base_engine::VoidEngine,
    constants::EngineResult,
};

mod engine;

#[async_std::main]
async fn main() -> EngineResult {
    let mut engine = VoidEngine::new();
    engine.run().await
}
