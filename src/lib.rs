use anyhow::Result;
use anyhow::bail as error_msg;
use serde::{Serialize, Deserialize};

mod fm_index;
mod structure;
mod generator;

mod config;

mod deprecated;
