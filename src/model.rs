use std::ops::Sub;

use chrono::NaiveTime;
use chrono::Timelike;
use clap::{command, Parser};
use color::HexColor;
use parser::weekday_ser;
use serde::{Deserialize, Serialize};
use serde_with::with_prefix;
use strum_macros::{Display, EnumIter};

use crate::color;
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, EnumIter, Display, Copy, Clone)]
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

impl Weekday {
  pub fn index(&self) -> usize {
    *self as usize
  }
}

with_prefix!(prefix_start "start");
with_prefix!(prefix_end "end");

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct MeetTime(pub NaiveTime);

impl MeetTime {
  pub fn new(hour: u32, minute: u32) -> Self {
    MeetTime(NaiveTime::from_hms_opt(hour, minute, 0).unwrap())
  }

  pub fn hour(&self) -> u32 {
    self.0.hour()
  }

  pub fn minute(&self) -> u32 {
    self.0.minute()
  }

  pub fn to_minutes(&self) -> u32 {
    self.hour() * 60 + self.minute()
  }

  pub fn to_hours(&self) -> f32 {
    self.hour() as f32 + self.minute() as f32 / 60.0
  }
}

impl Sub for MeetTime {
  type Output = Self;

  fn sub(self, rhs: Self) -> Self::Output {
    let minutes = self.to_minutes() - rhs.to_minutes();
    MeetTime::new(minutes / 60, minutes % 60)
  }
}

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
  pub title: String,
  pub items: Vec<ScheduleItem>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ScheduleSave {
  pub data_check: String,
  pub save_version: u32,
  pub schedules: Vec<Schedule>,
  pub current_schedule: u32,
}
