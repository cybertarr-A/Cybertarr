use attention::AttentionResult;
use chrono::Local;
use memory::MemoryStatistics;
use serde::{Deserialize, Serialize};
use state::{InternalState, Observation};

use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TickRecord {
    pub tick: u64,
    pub timestamp: String,

    pub observation: Observation,

    pub attention: AttentionResult,

    pub state: InternalState,

    pub memory: MemoryStatistics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    log: RunLog,
    output_directory: PathBuf,
    output_file: PathBuf,
}

impl Logger {
    pub fn new() -> Self {
        let now = Local::now();

        // Folder name (YYYY-MM-DD)
        let date = now.format("%Y-%m-%d").to_string();

        // File name
        let filename = now
            .format("experiment_%Y-%m-%d_%H-%M-%S.json")
            .to_string();

        // experiments/YYYY-MM-DD/
        let output_directory =
            PathBuf::from(format!("experiments/{}", date));

        create_dir_all(&output_directory)
            .expect("Failed to create experiments directory");

        let output_file = output_directory.join(filename);

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

            output_directory,
            output_file,
        }
    }

    pub fn record(
        &mut self,
        observation: &Observation,
        attention: &AttentionResult,
        state: &InternalState,
        memory: MemoryStatistics,
    ) {
        self.log.total_ticks = state.age_ticks;
        self.log.survived = state.alive;
        self.log.final_state = state.clone();

        self.log.ticks.push(TickRecord {
            tick: state.age_ticks,

            timestamp: Local::now().to_rfc3339(),

            observation: observation.clone(),

            attention: *attention,

            state: state.clone(),

            memory,
        });
    }

pub fn save(&mut self) {
    self.log.ended_at = Local::now().to_rfc3339();

    let json = serde_json::to_string_pretty(&self.log)
        .expect("Failed to serialize experiment");

    println!("Saving experiment to: {:?}", self.output_file);

    let mut file = File::create(&self.output_file)
        .expect("Failed to create experiment file");

    file.write_all(json.as_bytes())
        .expect("Failed to write experiment");

    println!("Experiment saved successfully.");
}

    #[allow(dead_code)]
    pub fn reset(&mut self) {
        *self = Logger::new();
    }

    pub fn experiment_directory(&self) -> &PathBuf {
        &self.output_directory
    }

    pub fn experiment_file(&self) -> &PathBuf {
        &self.output_file
    }
}