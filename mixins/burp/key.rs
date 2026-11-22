// The burp key went down or up: the player's choice of key, kept as
// `key.burp.burp` in `horizon-mods.txt`.
fn burp_key(game: &mut crate::game::Game<'_>, key: &str, pressed: bool) {
    use crate::game::burp as bp;
    // Turned off in Options > Mods: nothing until the next build leaves it out.
    if !game.native_on(bp::ID) {
        return;
    }
    // Only the press counts, and only while playing: keys reach this hook on
    // every screen, and typing the key in chat must not burp.
    if !pressed || !game.native_playing() {
        return;
    }
    let bound = game.native_mod_key(bp::ID, bp::KEY, bp::DEFAULT_KEY);
    if key != bound {
        return;
    }
    use horizon_core::random::RandomSource;
    let pitch = bp::key_burp_pitch(game.world.random.next_float());
    let at = game
        .player
        .as_ref()
        .map(|p| [p.position.x, p.position.y, p.position.z]);
    game.audio.play(bp::BIG_BURP, at, 1.0, pitch);
    log::info!("burp: key burp (pitch {pitch:.2})");
}
