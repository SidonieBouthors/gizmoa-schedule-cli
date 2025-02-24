use chrono::NaiveTime;
use clap::{command, Parser};
use hex_color::HexColor;
use parser::weekday_ser;
use serde::{Deserialize, Serialize};
use serde_with::with_prefix;
use strum_macros::{Display, EnumIter};

use crate::parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
  /// The path to the CSMO file
  pub csmo_file_path: String,

  /// Clock type: 12h or 24h
  #[arg(short, long)]
  pub clock_type: Option<String>,

  /// First day of week: Monday or Sunday
  #[arg(short, long)]
  pub first_weekday: Option<String>,

  /// Time increment: 30m or 1h
  #[arg(short, long)]
  pub time_increment: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, EnumIter, Display)]
#[serde(rename_all = "snake_case")]
pub enum Weekday {
  Monday,
  Tuesday,
  Wednesday,
  Thursday,
  Friday,
  Saturday,
  Sunday,
}

with_prefix!(prefix_start "start");
with_prefix!(prefix_end "end");

#[derive(Debug)]
pub struct MeetTime(pub NaiveTime);

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MeetingTime {
  uid: String,
  pub course_type: String,
  pub instructor: String,
  pub location: String,
  #[serde(flatten, with = "prefix_start")]
  pub start: MeetTime,
  #[serde(flatten, with = "prefix_end")]
  pub end: MeetTime,
  #[serde(with = "weekday_ser")]
  pub days: Vec<Weekday>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ScheduleItem {
  uid: String,
  #[serde(rename = "type")]
  pub item_type: String,
  pub title: String,
  pub meeting_times: Vec<MeetingTime>,
  pub background_color: HexColor,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Schedule {
  title: String,
  items: Vec<ScheduleItem>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ScheduleSave {
  data_check: String,
  save_version: u32,
  schedules: Vec<Schedule>,
  current_schedule: u32,
}
