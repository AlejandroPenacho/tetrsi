
mod game;
mod event;

use crossterm::terminal;
use crossterm::execute;

use std::sync::mpsc;
use std::thread;


fn main() {

    let mut game = start_game();

    game.draw_piece();

    let (sender, receiver) = mpsc::channel::<game::KeyOrder>();

    let event_sender = sender.clone();
    let auto_fall_sender = sender.clone();

    let key_handle = thread::spawn(move || event::event_loop(event_sender));
    let auto_fall_handle = thread::spawn(move || event::auto_fall(auto_fall_sender));

    loop {
        let next_event = receiver.recv().unwrap();

        if next_event == game::KeyOrder::Exit {
            break
        }
        
        match next_event {
            x if x == game::KeyOrder::Down => {game.move_piece((0, 1), 0, true);},
            x if x == game::KeyOrder::SoftDrop => {
                game.move_piece((0, 1), 0, true);
                game.update_score(1);
            },
            x if x == game::KeyOrder::HardDrop => {
                loop {
                    let has_fixed = game.move_piece((0, 1), 0, true);
                    game.update_score(2);

                    if has_fixed {
                        break
                    }
                }
            },
            x if x == game::KeyOrder::Left => {game.move_piece((-1, 0), 0, false);},
            x if x == game::KeyOrder::Right=> {game.move_piece((1, 0), 0, false);},
            x if x == game::KeyOrder::Rotate=> {game.move_piece((0, 0), 1, false);},
            _ => {}
        }
    }

    key_handle.join().unwrap();

    finish_game();

}

fn start_game() -> game::Game {
    let mut terminal_size = terminal::size().unwrap();

    terminal_size = (terminal_size.0.max(12+8), terminal_size.1.max(22+1+3));
    
    let mut stdout = std::io::stdout();

    execute!(
        stdout, 
        terminal::EnterAlternateScreen,
        terminal::SetSize(terminal_size.0, terminal_size.1),
        crossterm::cursor::Hide
    ).unwrap();

    terminal::enable_raw_mode().unwrap();

    let game = game::Game::new();
    game.draw_board();

    game
}

fn finish_game() {
    terminal::disable_raw_mode().unwrap();

    execute!(std::io::stdout(), 
        terminal::LeaveAlternateScreen,
        crossterm::cursor::Show
    ).unwrap();
}