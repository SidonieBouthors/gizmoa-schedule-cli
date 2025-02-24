use std::fs;

use clap::Parser;
use model::{Args, ScheduleSave};

pub mod model;
mod parser;

fn main() {
  let args = Args::parse();
  let json_content =
    fs::read_to_string(args.csmo_file_path).expect("Should have been able to read the file");
  let schedule_save: ScheduleSave =
    serde_json::from_str(&json_content).expect("JSON was not well-formatted");

  print!("{:?}", schedule_save);
}
