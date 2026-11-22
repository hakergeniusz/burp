// Burp Mod, a native mod: a rebindable key that burps on demand, and a big
// burp a beat after finishing a meal. This file is both the mod's Cargo
// library (its tests) and the module the bundle adds to the game.
//
// Horizon runs two Games in single player (the window's and the single
// player server's) and `native_tick` fires in both, so the pending big
// burp's countdown lives in a static the eat hook (which runs on the
// server's Game, where the use completes) hands to the window's tick, the
// only Game with the local player's position and audio.
use std::cell::RefCell;
use std::sync::atomic::{AtomicU32, Ordering};

/// The mod's id (its settings folder and its keys' lines), name and version.
pub const ID: &str = "burp";
pub const NAME: &str = "Burp Mod";
pub const VERSION: &str = "1.0.0";
pub const DESCRIPTION: &str = "Burp on demand with a key you can rebind, and let out a big burp a \
beat after finishing a meal. Vanilla's own small burp still plays — this is the big one on top.";

/// The burp key's name, and its default.
pub const KEY: &str = "burp";
pub const DEFAULT_KEY: &str = "key.keyboard.b";

/// The big burp: the player's own recording, wired into the prepared
/// assets as a `burpmod` sound event (assets/burpmod/sounds.json). Both
/// burps play it — the key one near vanilla pitch, the after-meal one deep.
pub const BIG_BURP: &str = "burpmod:big_burp";

/// The big burp plays this many ticks (0.6 s) after the meal is finished.
pub const BIG_BURP_DELAY_TICKS: u32 = 12;

/// Ticks until the pending big burp fires; 0 when nothing is pending.
/// Written by the eat hook (on the server's Game) and counted down by the
/// window's tick, so it must be shared across threads.
pub static PENDING: AtomicU32 = AtomicU32::new(0);

/// The key burp's pitch, from one `next_float()`: vanilla's 0.9–1.0
/// stretched a little wider, so no two burps sound the same.
pub fn key_burp_pitch(random: f32) -> f32 {
    0.85 + random.clamp(0.0, 1.0) * 0.35
}

/// The big burp's pitch, from one `next_float()`: deep and random.
pub fn big_burp_pitch(random: f32) -> f32 {
    0.55 + random.clamp(0.0, 1.0) * 0.2
}

/// The meal is finished (the consumable had food): schedule the big burp
/// unless turned off or one is already on the way. True when scheduled.
pub fn schedule(clock: &AtomicU32, on: bool) -> bool {
    on && clock
        .compare_exchange(
            0,
            BIG_BURP_DELAY_TICKS,
            Ordering::Relaxed,
            Ordering::Relaxed,
        )
        .is_ok()
}

/// One window tick. True on the tick the pending big burp should play.
pub fn tick_pending(clock: &AtomicU32) -> bool {
    loop {
        let left = clock.load(Ordering::Relaxed);
        if left == 0 {
            return false;
        }
        match clock.compare_exchange(left, left - 1, Ordering::Relaxed, Ordering::Relaxed) {
            Ok(_) => return left == 1,
            Err(_) => continue,
        }
    }
}

/// A pending burp is dropped (the mod was turned off mid-countdown).
pub fn cancel(clock: &AtomicU32) {
    clock.store(0, Ordering::Relaxed);
}

/// The mod's state: the on/off toggle, kept as one key under
/// `horizon-mods/.horizon-mod-data/burp/`, so a player keeps it between
/// sessions.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct State {
    pub on: bool,
}

impl State {
    /// From the stored values; on when missing.
    pub fn read(get: impl Fn(&str) -> Option<String>) -> Self {
        let on = !get("on").is_some_and(|v| v.trim() == "false");
        Self { on }
    }

    /// The settings page's buttons.
    pub fn labels(&self) -> Vec<String> {
        let on = if self.on { "ON" } else { "OFF" };
        vec![format!("Burp Mod: {on}")]
    }

    /// A click on button `i` of `labels`. True when the toggle changed.
    pub fn click(&mut self, i: usize) -> bool {
        if i == 0 {
            self.on = !self.on;
            return true;
        }
        false
    }

    /// What to store, key by key.
    pub fn stored(&self) -> [(&'static str, String); 1] {
        [("on", if self.on { "true".into() } else { "false".into() })]
    }
}

thread_local! {
    /// The state, read once from the files, then kept (per Game: the
    /// window's copy drives the settings page and the key).
    static STATE: RefCell<Option<State>> = const { RefCell::new(None) };
}

/// The state, read once through `get` (one file per key).
pub fn state(get: impl Fn(&str) -> Option<String>) -> State {
    STATE.with_borrow_mut(|s| *s.get_or_insert_with(|| State::read(get)))
}

/// Replaces the state (the settings page changed it).
pub fn set_state(state: State) {
    STATE.with_borrow_mut(|s| *s = Some(state));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn stored<'a>(pairs: &'a [(&'a str, &'a str)]) -> impl Fn(&str) -> Option<String> + 'a {
        |key| pairs.iter().find(|p| p.0 == key).map(|p| p.1.to_owned())
    }

    #[test]
    fn defaults_to_on() {
        assert!(State::read(stored(&[])).on);
        assert_eq!(State::read(stored(&[])).labels(), ["Burp Mod: ON"]);
    }

    #[test]
    fn reads_the_stored_state() {
        assert!(!State::read(stored(&[("on", "false\n")])).on);
        assert!(State::read(stored(&[("on", "true\n")])).on);
        assert!(State::read(stored(&[("on", "nonsense")])).on);
    }

    #[test]
    fn clicks_toggle() {
        let mut s = State::read(stored(&[]));
        assert!(s.click(0));
        assert!(!s.on);
        assert!(s.click(0));
        assert!(s.on);
        assert!(!s.click(1));
        assert_eq!(s.stored(), [("on", "true".into())]);
    }

    #[test]
    fn a_meal_schedules_one_big_burp_when_on() {
        let clock = AtomicU32::new(0);
        assert!(schedule(&clock, true));
        assert!(!schedule(&clock, true));
        assert_eq!(clock.load(Ordering::Relaxed), BIG_BURP_DELAY_TICKS);
    }

    #[test]
    fn no_big_burp_when_turned_off() {
        let clock = AtomicU32::new(0);
        assert!(!schedule(&clock, false));
        assert_eq!(clock.load(Ordering::Relaxed), 0);
    }

    #[test]
    fn the_big_burp_fires_after_the_delay() {
        let clock = AtomicU32::new(0);
        schedule(&clock, true);
        for _ in 0..BIG_BURP_DELAY_TICKS - 1 {
            assert!(!tick_pending(&clock));
        }
        assert!(tick_pending(&clock));
        assert_eq!(clock.load(Ordering::Relaxed), 0);
        assert!(!tick_pending(&clock));
    }

    #[test]
    fn ticks_do_nothing_without_a_pending_burp() {
        let clock = AtomicU32::new(0);
        assert!(!tick_pending(&clock));
        assert!(!tick_pending(&clock));
    }

    #[test]
    fn cancel_drops_a_pending_burp() {
        let clock = AtomicU32::new(0);
        schedule(&clock, true);
        cancel(&clock);
        assert!(!tick_pending(&clock));
    }

    #[test]
    fn key_burps_span_vanilla_pitch_and_a_bit_more() {
        assert_eq!(key_burp_pitch(0.0), 0.85);
        assert_eq!(key_burp_pitch(1.0), 1.2);
        assert!(key_burp_pitch(-3.0) >= 0.85);
        assert!(key_burp_pitch(7.0) <= 1.2);
    }

    #[test]
    fn big_burps_run_deep() {
        assert_eq!(big_burp_pitch(0.0), 0.55);
        assert_eq!(big_burp_pitch(1.0), 0.75);
        assert!(big_burp_pitch(0.5) > 0.55 && big_burp_pitch(0.5) < 0.75);
    }
}
