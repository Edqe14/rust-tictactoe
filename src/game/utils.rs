use std::io::{stdin};
use console::{style, StyledObject};

pub fn ask_user() -> String {
  let mut txt = String::new();
  stdin().read_line(&mut txt).unwrap();

  txt.trim_end().to_string()
}

pub fn format_player(player: &char) -> StyledObject<&char> {
  let val;
  if player == &'X' {
    val = style(player).green();
  } else if player == &'O' {
    val = style(player).yellow();
  } else {
    val = style(player).red();
  }

  val.bold()
}