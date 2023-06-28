//! Definitions for the [TickCounter] struct.

use std::time::Instant;

pub struct TickCounter {
    start: Instant,
    nanos_per_tick: u128,
    tick: u128,
}

/// Keeps track of time throughout a game session.
impl TickCounter {
    pub fn start(ticks_per_second: u128) -> Self {
        TickCounter {
            start: Instant::now(),
            nanos_per_tick: 1_000_000_000 / ticks_per_second,
            tick: 0,
        }
    }

    pub fn is_next_tick(&mut self) -> bool {
        let curr =
            self.start.elapsed().as_nanos().saturating_div(self.nanos_per_tick);

        if curr > self.tick {
            self.tick = curr;
            return true;
        }

        false
    }
}
