use strum::IntoEnumIterator;
use svg::{
  node::element::{Line, Rectangle, Text},
  Document,
};

use crate::model::{MeetTime, Schedule};
use crate::model::{ScheduleItem, Weekday};

struct ScheduleConfig {
  save_name: String,
  days: Vec<String>,
  slot_height: f32,
  slot_width: f32,
  time_margin: f32,
  day_margin: f32,
  outer_margin: f32,
  font_size: f32,
  small_font_size: f32,
}

fn default_config() -> ScheduleConfig {
  ScheduleConfig {
    save_name: "schedule".to_string(),
    days: Weekday::iter().map(|d| d.to_string()).collect(),
    slot_height: 50.0,
    slot_width: 100.0,
    time_margin: 50.0,
    day_margin: 50.0,
    outer_margin: 10.0,
    font_size: 10.0,
    small_font_size: 10.0,
  }
}

pub fn save_vector_schedule(schedule: &Schedule) {
  let schedule_config = default_config();

  let (start_hour, end_hour) = calculate_start_end_hours(schedule);

  let width = schedule_config.time_margin
    + schedule_config.outer_margin * 2.0
    + schedule_config.slot_width * schedule_config.days.len() as f32;
  let height = schedule_config.day_margin
    + schedule_config.outer_margin * 2.0
    + schedule_config.slot_height * (end_hour - start_hour) as f32;

  let mut document = Document::new().set("viewBox", (0, 0, width, height));

  document = draw_grid(document, &schedule_config, (start_hour, end_hour));

  document = draw_time_labels(document, &schedule_config, (start_hour, end_hour));

  document = draw_day_headers(document, &schedule_config);

  for item in &schedule.items {
    document = draw_event(document, &item, &schedule_config, (start_hour, end_hour));
  }

  svg::save(schedule_config.save_name + ".svg", &document).unwrap();
}

/// Draw the schedule grid
fn draw_grid(
  mut document: Document,
  config: &ScheduleConfig,
  (start_hour, end_hour): (u32, u32),
) -> Document {
  // Calculate dimensions
  let grid_width = config.slot_width * config.days.len() as f32;
  let grid_height = config.slot_height * (end_hour - start_hour) as f32;
  let start_x = config.outer_margin + config.time_margin;
  let start_y = config.outer_margin + config.day_margin; // Space for headers

  // Draw horizontal time slot lines
  for hour in start_hour..=end_hour {
    let y = start_y + (hour - start_hour) as f32 * config.slot_height;
    let line = Line::new()
      .set("x1", start_x)
      .set("y1", y)
      .set("x2", start_x + grid_width)
      .set("y2", y)
      .set("stroke", "black")
      .set("stroke-width", 1);
    document = document.add(line);
  }

  // Draw vertical day column lines
  for day in 0..=config.days.len() {
    let x = start_x + day as f32 * config.slot_width;
    let line = Line::new()
      .set("x1", x)
      .set("y1", start_y)
      .set("x2", x)
      .set("y2", start_y + grid_height)
      .set("stroke", "black")
      .set("stroke-width", 1);
    document = document.add(line);
  }

  document
}

fn draw_time_labels(
  mut document: Document,
  config: &ScheduleConfig,
  (start_hour, end_hour): (u32, u32),
) -> Document {
  let start_y = config.outer_margin + config.day_margin; // Space for headers

  for hour in start_hour..end_hour {
    let y = start_y + (hour - start_hour) as f32 * config.slot_height;

    // Format time as HH:MM in 24-hour format
    let time_str = format!("{:02}:00", hour);

    // Draw time label
    document = draw_text(
      document,
      &time_str,
      config.time_margin + config.outer_margin - 5.0,
      y + 15.0,
      config.font_size,
      true,
      "end",
    );
  }

  document
}

// Draw the day headers at the top
fn draw_day_headers(mut document: Document, config: &ScheduleConfig) -> Document {
  let start_x = config.outer_margin + config.time_margin;
  let header_y = config.outer_margin + config.day_margin / 2.0;

  for (i, day) in config.days.iter().enumerate() {
    let x = start_x + i as f32 * config.slot_width + config.slot_width / 2.0; // ???

    // Draw day header
    document = draw_text(document, day, x, header_y, config.font_size, true, "middle");
  }

  document
}

/// Draw a single event on the schedule
fn draw_event(
  mut document: Document,
  item: &ScheduleItem,
  config: &ScheduleConfig,
  (start_hour, _end_hour): (u32, u32),
) -> Document {
  for slot in &item.meeting_times {
    let start_minutes = slot.start.to_minutes() as f32;
    let end_minutes = slot.end.to_minutes() as f32;

    let start_offset_minutes = start_minutes - (start_hour as f32 * 60.0);
    let duration_minutes = end_minutes - start_minutes;

    let start_y =
      config.outer_margin + config.day_margin + (start_offset_minutes / 60.0) * config.slot_height;
    let height = (duration_minutes / 60.0) * config.slot_height;

    for day in &slot.days {
      let start_x =
        config.outer_margin + config.time_margin + day.index() as f32 * config.slot_width;

      // Draw slot rectangle
      let rect = Rectangle::new()
        .set("x", start_x)
        .set("y", start_y)
        .set("width", config.slot_width)
        .set("height", height)
        .set("fill", item.background_color.to_hex())
        .set("stroke", "black")
        .set("stroke-width", 1);

      document = document.add(rect);

      // Add event title
      document = draw_text(
        document,
        &item.title,
        start_x + 5.0,
        start_y + 15.0,
        config.font_size,
        true,
        "start",
      );

      // Add time information
      let time_text = format!(
        "{}:{:02}-{}:{:02}",
        slot.start.hour(),
        slot.start.minute(),
        slot.end.hour(),
        slot.end.minute()
      );

      // Add time
      document = draw_text(
        document,
        &time_text,
        start_x + 5.0,
        start_y + 30.0,
        config.small_font_size,
        false,
        "start",
      );

      // Add location
      document = draw_text(
        document,
        &slot.location,
        start_x + 5.0,
        start_y + 45.0,
        config.small_font_size,
        false,
        "start",
      );

      // Add instructor
      document = draw_text(
        document,
        &slot.instructor,
        start_x + 5.0,
        start_y + 60.0,
        config.small_font_size,
        false,
        "start",
      );
    }
  }
  document
}

fn draw_text(
  document: Document,
  text: &str,
  x: f32,
  y: f32,
  font_size: f32,
  bold: bool,
  text_anchor: &str,
) -> Document {
  let mut text = Text::new(text)
    .set("x", x)
    .set("y", y)
    .set("font-size", font_size)
    .set("font-family", "sans-serif")
    .set("text-anchor", text_anchor);

  if bold {
    text = text.set("font-weight", "bold");
  }

  document.add(text)
}

fn calculate_start_end_hours(schedule: &Schedule) -> (u32, u32) {
  let mut start_time = MeetTime::new(23, 59);
  let mut end_time = MeetTime::new(0, 0);

  for item in &schedule.items {
    for slot in &item.meeting_times {
      if slot.start < start_time {
        start_time = slot.start;
      }
      if slot.end > end_time {
        end_time = slot.end;
      }
    }
  }

  (
    start_time.to_hours().floor() as u32,
    end_time.to_hours().ceil() as u32,
  )
}
