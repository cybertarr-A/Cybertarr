use chrono::Local;
use serde::Serialize;
use std::fs::{create_dir_all, File};
use std::io::Write;
use uuid::Uuid;

use crate::observation::Observation;
use crate::state::InternalState;

#[derive(Serialize)]
pub struct TickRecord {
    pub tick: u64,
    pub observation: Observation,
    pub state: InternalState,
}

#[derive(Serialize)]
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
}

impl Logger {
    pub fn new() -> Self {
        Self {
            log: RunLog {
                run_id: Uuid::new_v4().to_string(),

                started_at: Local::now().to_rfc3339(),
                ended_at: String::new(),

                version: "0.0.1-alpha".into(),

                total_ticks: 0,
                survived: true,

                final_state: InternalState::default(),

                ticks: Vec::new(),
            },
        }
    }

    pub fn record(
        &mut self,
        observation: &Observation,
        state: &InternalState,
    ) {
        self.log.total_ticks = state.age_ticks;
        self.log.survived = state.alive;
        self.log.final_state = state.clone();

        self.log.ticks.push(TickRecord {
            tick: state.age_ticks,
            observation: observation.clone(),
            state: state.clone(),
        });
    }

    pub fn save(&mut self) {
        create_dir_all("experiments").unwrap();

        self.log.ended_at = Local::now().to_rfc3339();

        let filename = format!(
            "experiments/{}.json",
            self.log.run_id
        );

        let json =
            serde_json::to_string_pretty(&self.log).unwrap();

        let mut file = File::create(filename).unwrap();

        file.write_all(json.as_bytes()).unwrap();
    }
}