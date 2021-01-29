use std::fs;
use crate::{config::Check};

/// Writes to Scoring Report
pub fn write_to_scoring_report (file: &String, scored_vector: impl Iterator<Item = Check>) {
  let mut text = String::from("<ul>\n");
  for scored in scored_vector {
    text.push_str(&format!("<li>Check passed: {} - {} points</li>\n", scored.name, scored.points));
  }
  text.push_str(&"</ul>\n");
  let f = fs::write(file, text);
  match f {
    Err(error) => println!("Problem opening the file: {:?}", error),
    Ok(_res) => println!("Wrote to file successfully")
  };
}