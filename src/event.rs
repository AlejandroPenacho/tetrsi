use super::game::KeyOrder;

use std::sync::mpsc;

use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEvent};


pub fn event_loop(sender: mpsc::Sender<KeyOrder>) {

    let down_key: Event = Event::Key(KeyEvent::from(KeyCode::Down));
    let up_key: Event = Event::Key(KeyEvent::from(KeyCode::Up));
    let right_key: Event = Event::Key(KeyEvent::from(KeyCode::Right));
    let left_key: Event = Event::Key(KeyEvent::from(KeyCode::Left));
    let exit_key: Event = Event::Key(KeyEvent::from(KeyCode::Char('q')));


    loop {
        let event = event::read().unwrap();

        let out = match event {
            x if x == down_key => Some(KeyOrder::Down),
            x if x == up_key => Some(KeyOrder::Up),
            x if x == left_key => Some(KeyOrder::Left),
            x if x == right_key => Some(KeyOrder::Right),
            x if x == exit_key => Some(KeyOrder::Exit),
            _ => {None}
        };

        if let Some(order) = out {

            let exit = order == KeyOrder::Exit;

            sender.send(order).unwrap();

            if exit {
                break
            }
        }
    }


}