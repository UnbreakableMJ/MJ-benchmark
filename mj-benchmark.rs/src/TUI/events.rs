// SPDX-License-Identifier: GPL-3.0-or-later
//
// MJ Benchmark
// Copyright (c) 2024-2026
// Mohamed Hammad
//
// Trademarks:
//   Steelbore, S3cure, S3cure me, S3cure us, MJ Benchmark
//   These names are trademarks of Mohamed Hammad and may not be used
//   to endorse or promote derivative products without prior permission.

use crossterm::event::{self, Event, KeyCode};
use std::time::Duration;

#[derive(Debug)]
pub enum TuiEvent {
    Tick,
    Key(KeyCode),
}

pub fn poll_event(timeout: Duration) -> Option<TuiEvent> {
    if event::poll(timeout).ok()? {
        if let Ok(ev) = event::read() {
            match ev {
                Event::Key(k) => Some(TuiEvent::Key(k.code)),
                _ => None,
            }
        } else {
            None
        }
    } else {
        Some(TuiEvent::Tick)
    }
}