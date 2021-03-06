use console::{style};

pub mod utils;

pub fn new() {
  let mut board: [char; 9] = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];
  let mut player = 'X';

  loop {
    draw(&board);
    step(&mut board, player);
    if has_won(&board) {
      println!("{} {}", utils::format_player(&player), style("won! \\o/").blue());
      break;
    }

    if board.iter().all(|&v| v == 'X' || v == 'O') {
      println!("{}", style("It's a tie!").bold().italic().blue());
      break;
    }

    player = if player == 'X' { 'O' } else { 'X' };
  }
}

fn has_won(board: &[char; 9]) -> bool {
  for tmp in 0..3 {
    if board[tmp] == board[tmp + 3] && board[tmp] == board[tmp + 6] {
      return true;
    }

    let tmp = tmp * 3;
    if board[tmp] == board[tmp + 1] && board[tmp] == board[tmp + 2] {
      return true;
    }
  }

  if (board[0] == board[4] && board[0] == board[8]) || (board[2] == board[4] && board[2] == board[6]) {
      return true;
  }

  false
}

fn step(board: &mut [char; 9], player: char) {
  println!("{}: What's your pick? (1-9)", utils::format_player(&player));

  let mut input;
  loop {
    input = utils::ask_user().trim().parse::<usize>();
    if let Ok(number) = input {
      if ((number as i32) < 0) || (number as i32 > board.len() as i32) {
        println!("{}", style("Unknown slot!").red());
        continue;
      }

      let number = number - 1;

      if board[number] == 'X' || board[number] == 'O' {
        println!("{} {}", style("This slot is already taken by").red(), utils::format_player(&board[number]));
        continue;
      }

      board[number] = player;
      clear_terminal();
      break;
    } else {
      println!("Your input is not a number!");
      continue;
    }
  }
}

fn draw(board: &[char; 9]) {
  for i in 0..3 {
    let offset = i * 3;

    println!(
      "-------------\n| {} | {} | {} |",
      utils::format_player(&board[offset]),
      utils::format_player(&board[offset + 1]),
      utils::format_player(&board[offset + 2])
    )
  }

  println!("-------------");
}

pub fn clear_terminal() {
  print!("\x1B[2J\x1B[1;1H");
}