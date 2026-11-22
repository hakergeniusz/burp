// One window tick: the burp key's click (the registered `burp` mapping's
// click, polled here) plays a burp at a random pitch, and the pending big
// burp fires when its countdown is up. The integrated server's Game ticks
// too and the eat hook runs there, so this is the window's Game only —
// the one with the local player's position and audio. The stored toggle
// refreshes into the brain every tick, so a change on the mod's page
// lands without a restart.
// Copyright (c) 2026 hakergeniusz. MIT.
fn burp_tick(game: &mut crate::game::Game<'_>) {
    if !game.options.execution_role.is_client() {
        return;
    }
    // The stored toggle first: off drops a pending burp, as the old
    // disable branch did.
    let on = game
        .stored_setting(burp::ID, "on")
        .is_none_or(|v| v.trim() != "false");
    burp::set_state(burp::State { on });
    if !on {
        burp::cancel(&burp::PENDING);
        return;
    }
    // The key burp's click (a click never comes from behind a screen, so
    // typing the key in chat cannot burp), and the countdown's fire.
    let key_burp = game.consume_click(burp::ID, burp::KEY);
    let big_burp = burp::tick_pending(&burp::PENDING);
    if !key_burp && !big_burp {
        return;
    }
    use horizon_core::random::RandomSource;
    let pitch = if key_burp {
        burp::key_burp_pitch(game.world.random.next_float())
    } else {
        burp::big_burp_pitch(game.world.random.next_float())
    };
    let at = game
        .player
        .as_ref()
        .map(|p| [p.position.x, p.position.y, p.position.z]);
    game.audio.play(burp::BIG_BURP, at, 1.0, pitch);
}
