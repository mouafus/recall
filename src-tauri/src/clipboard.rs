use arboard::Clipboard;
use crate::commands::HISTORY;
use clipboard_master::{CallbackResult, ClipboardHandler, Master};
use std::{io, thread};
use base64::Engine;

pub struct Handler {
    clipboard: Clipboard,
}

impl Handler {
    pub fn new() -> Result<Self, arboard::Error> {
        Ok(Self {
            clipboard: Clipboard::new()?,
        })
    }

    fn read_text(&mut self) -> Option<(String, String)> {
        if let Ok(text) = self.clipboard.get_text() {
            let lower = text.to_lowercase();
            let trimmed = lower.trim_start();
            let content_type = if trimmed.starts_with("{\\rtf") {
                "text/rtf"
            } else if lower.contains("<html")
                || lower.contains("<body")
                || lower.contains("<div")
                || lower.contains("<p>")
                || lower.contains("<span")
            {
                "text/html"
            } else {
                "text/plain"
            };
            return Some((text, content_type.to_string()));
        }
        None
    }
}

impl ClipboardHandler for Handler {
    fn on_clipboard_change(&mut self) -> CallbackResult {
        // First try image
        if let Ok(img) = self.clipboard.get_image() {
            let content = format!("[image {}x{}]", img.width, img.height);
            if !content.is_empty() {
                let image_base64 = Some(base64::engine::general_purpose::STANDARD.encode(&img.bytes));
                HISTORY.remove_by_content(&content);
                HISTORY.add(
                    content,
                    "image".to_string(),
                    image_base64,
                    Some(img.width as u32),
                    Some(img.height as u32),
                );
            }
            return CallbackResult::Next;
        }

        // Fallback to text
        if let Some((content, content_type)) = self.read_text() {
            if !content.is_empty() {
                HISTORY.remove_by_content(&content);
                HISTORY.add(content.clone(), content_type, None, None, None);
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
        match Handler::new() {
            Ok(handler) => {
                let _ = Master::new(handler).run();
            }
            Err(err) => {
                eprintln!("Failed to create clipboard handler: {}", err);
            }
        }
    })
}