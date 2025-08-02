use arboard::Clipboard;

use crate::commands::HISTORY;
use clipboard_master::{CallbackResult, ClipboardHandler, Master};
use std::{io, thread};

pub fn read_clipboard() -> Option<String> {
    let mut clipboard = Clipboard::new().ok()?;
    clipboard.get_text().ok()
}

pub struct Handler;

impl ClipboardHandler for Handler {
    fn on_clipboard_change(&mut self) -> CallbackResult {
        if let Some(content) = read_clipboard() {
            if !content.is_empty() {
                HISTORY.add(content.clone(), "text/plain".to_string());
            }
        }
        CallbackResult::Next
    }

    fn on_clipboard_error(&mut self, error: io::Error) -> CallbackResult {
        eprintln!("Error: {}", error);
        CallbackResult::Next
    }
}

pub fn start_clipboard_watcher() -> thread::JoinHandle<()> {
    thread::spawn(|| {
        println!("Thread started");
        let _ = Master::new(Handler).run();
    })
}
