// One window tick: fire the pending big burp when its time is up. The
// server's Game ticks too (native_tick runs in every Game) and the eat
// hook runs there, so this is the window's Game only — the one with the
// local player's position and audio.
fn burp_tick(game: &mut crate::game::Game<'_>) {
    use crate::game::burp as bp;
    if !game.native_window() {
        return;
    }
    // Turned off in Options > Mods: nothing until the next build leaves it out.
    if !game.native_on(bp::ID) {
        return;
    }
    if !bp::tick_pending(&bp::PENDING) {
        return;
    }
    use horizon_core::random::RandomSource;
    let pitch = bp::big_burp_pitch(game.world.random.next_float());
    let at = game
        .player
        .as_ref()
        .map(|p| [p.position.x, p.position.y, p.position.z]);
    game.audio.play(bp::BIG_BURP, at, 1.0, pitch);
    log::info!("burp: big burp after the meal (pitch {pitch:.2})");
}
