use std::sync::mpsc::{channel,Receiver};

pub fn open_public_stream() -> Receiver<String> {
    let (tx, rx) = channel::<String>();

    rx
}
