use anyhow::Result;
use anyhow::bail as error_msg;
use serde::{Serialize, Deserialize};

mod fm_index;
mod algorithm;
mod builder;

mod config;

pub mod deprecated;
