use super::game::KeyOrder;

use std::sync::mpsc;
use std::thread::sleep;
use std::time::Duration;

use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEvent};


pub fn auto_fall(sender: mpsc::Sender<KeyOrder>) {
    loop {
        sleep(Duration::from_millis(500));
        let send_status = sender.send(KeyOrder::Down);

        if let Err(_) = send_status {
            break
        }
    }
}

pub fn event_loop(sender: mpsc::Sender<KeyOrder>) {

    let down_key: Event = Event::Key(KeyEvent::from(KeyCode::Down));
    let up_key: Event = Event::Key(KeyEvent::from(KeyCode::Up));
    let right_key: Event = Event::Key(KeyEvent::from(KeyCode::Right));
    let left_key: Event = Event::Key(KeyEvent::from(KeyCode::Left));
    let exit_key: Event = Event::Key(KeyEvent::from(KeyCode::Char('q')));
    let rotate_key: Event = Event::Key(KeyEvent::from(KeyCode::Char('z')));

    loop {
        let event = event::read().unwrap();

        let out = match event {
            _ if event == down_key => Some(KeyOrder::SoftDrop),
            _ if event == up_key => Some(KeyOrder::HardDrop),
            _ if event == left_key => Some(KeyOrder::Left),
            _ if event == right_key => Some(KeyOrder::Right),
            _ if event == exit_key => Some(KeyOrder::Exit),
            _ if event == rotate_key => Some(KeyOrder::Rotate),
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
