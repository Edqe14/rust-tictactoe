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

  println!("{}\n1. New game\n2. Exit", style("Menu").bold().underlined());

  let pick = game::ask_user();
  if pick == "1" {
    loop {
      game::clear_terminal();
      game::new();

      println!("Game over! Play again? (Y/n)");
      let mut retry: String;
      loop {
        retry = game::ask_user().to_lowercase().trim().to_string();
        if retry == "" {
          retry = "y".to_string();
        } else if retry != "n" && retry != "y" {
          continue;
        }

        break;
      }

      if retry == "y" {
        continue;
      }

      goodbye();
      break;
    }
  } else {
    goodbye();
  }
}
