use attention::AttentionResult;
use chrono::Local;
use serde::Serialize;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::PathBuf;

use crate::observation::Observation;
use crate::state::InternalState;

#[derive(Debug, Clone, Serialize)]
pub struct TickRecord {
    pub tick: u64,
    pub observation: Observation,
    pub attention: AttentionResult,
    pub state: InternalState,
}

#[derive(Debug, Clone, Serialize)]
pub struct RunLog {
    pub run_id: String,
    pub started_at: String,
    pub ended_at: String,
    pub version: String,
    pub total_ticks: u64,
    pub survived: bool,
    pub final_state: InternalState,
    pub ticks: Vec<TickRecord>,
}

pub struct Logger {
    pub log: RunLog,
    output_file: PathBuf,
}

impl Logger {
    pub fn new() -> Self {
        let now = Local::now();

        let date = now.format("%Y-%m-%d").to_string();
        let timestamp = now
            .format("experiment_%Y-%m-%d_%H-%M-%S")
            .to_string();

        let directory = format!("experiments/{}", date);

        create_dir_all(&directory).unwrap();

        let output_file =
            PathBuf::from(format!("{}/{}.json", directory, timestamp));

        Self {
            log: RunLog {
                run_id: now.format("%Y%m%d%H%M%S").to_string(),
                started_at: now.to_rfc3339(),
                ended_at: String::new(),
                version: "0.0.1-alpha".into(),
                total_ticks: 0,
                survived: true,
                final_state: InternalState::default(),
                ticks: Vec::new(),
            },
            output_file,
        }
    }

    pub fn record(
        &mut self,
        observation: &Observation,
        attention: &AttentionResult,
        state: &InternalState,
    ) {
        self.log.total_ticks = state.age_ticks;
        self.log.survived = state.alive;
        self.log.final_state = state.clone();

        self.log.ticks.push(TickRecord {
            tick: state.age_ticks,
            observation: observation.clone(),
            attention: attention.clone(),
            state: state.clone(),
        });
    }

    pub fn save(&mut self) {
        self.log.ended_at = Local::now().to_rfc3339();

        let json =
            serde_json::to_string_pretty(&self.log).unwrap();

        let mut file =
            File::create(&self.output_file).unwrap();

        file.write_all(json.as_bytes()).unwrap();
    }
}