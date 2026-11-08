//! A display-only FPS number for Horizon's HUD.
//!
//! This deliberately does not read or change Horizon's measured frame rate.

use std::cell::Cell;
use std::time::Duration;

pub const ID: &str = "fps_display_spoofer";
pub const NAME: &str = "FPS Display Spoofer";
pub const VERSION: &str = "1.0.0";
pub const DESCRIPTION: &str = "A display-only FPS counter that cycles between 13,000 and 28,000.";
pub const BACKGROUND: u32 = 0x7800_0000;
pub const INK: u32 = 0xFF_FFFF;
pub const CORNERS: [(&str, &str); 4] = [
    ("top_left", "Top left"),
    ("top_right", "Top right"),
    ("bottom_left", "Bottom left"),
    ("bottom_right", "Bottom right"),
];

#[derive(Clone, Copy)]
struct DisplayState {
    seed: u32,
    elapsed_ms: u128,
    fps: u32,
}

thread_local! {
    static DISPLAY: Cell<DisplayState> = const { Cell::new(DisplayState {
        seed: 0x5EED_C0DE,
        elapsed_ms: 0,
        fps: 20_480,
    }) };
    static CORNER: Cell<Option<usize>> = const { Cell::new(None) };
}

/// Advances the displayed value at most about three times per second.
pub fn advance(elapsed: Duration) {
    DISPLAY.with(|display| {
        let mut state = display.get();
        state.elapsed_ms += elapsed.as_millis();
        if state.elapsed_ms >= 350 {
            state.elapsed_ms %= 350;
            state.seed = state.seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            state.fps = 13_000 + state.seed % 15_001;
        }
        display.set(state);
    });
}

pub fn fps() -> u32 {
    DISPLAY.with(|display| display.get().fps)
}

pub fn corner(read: impl FnOnce() -> Option<String>) -> usize {
    CORNER.with(|corner| {
        let value = corner.get().unwrap_or_else(|| {
            read()
                .and_then(|saved| CORNERS.iter().position(|(key, _)| *key == saved.trim()))
                .unwrap_or(0)
        });
        corner.set(Some(value));
        value
    })
}

pub fn cycle_corner() -> usize {
    CORNER.with(|corner| {
        let next = (corner.get().unwrap_or(0) + 1) % CORNERS.len();
        corner.set(Some(next));
        next
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generated_readings_stay_in_the_requested_range() {
        for _ in 0..100 {
            advance(Duration::from_millis(350));
            assert!((13_000..=28_000).contains(&fps()));
        }
    }
}
