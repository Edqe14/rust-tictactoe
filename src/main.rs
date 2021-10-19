use console::{style};

mod game;

fn greeting() {
  println!("{}", style("Tic-Tac-Toe").cyan().bold().dim());
  println!("A basic minigame build using Rust!\n");
}

fn goodbye() {
  game::clear_terminal();
  println!("{}", style("See you next time! o/").magenta().bold());

  println!("{}", style("Press any key to continue...").dim().italic());
  game::ask_user();
}

fn main() {
  greeting();

  println!("{}\n1. New game\n2. Exit", style("Menu").green().bold().underlined());

  let pick = game::ask_user();
  if pick == "1" {
    loop {
      game::clear_terminal();
      game::new();

      println!("{} (Y/n)", style("Game over! Play again?").bold().italic());
      let retry = loop {
        match game::ask_user().to_lowercase().trim() {
          "" | "y" => break true,
          "n" => break false,
          _ => println!("{}", style("Unknown option!").red()),
        }
      };

      if retry {
        continue;
      }

      goodbye();
      break;
    }
  } else {
    goodbye();
  }
}
