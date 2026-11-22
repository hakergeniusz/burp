// A click on the mod's settings page: change the state and keep it. The
// server's Game reads the same setting when a meal finishes, so the file
// is the source of truth; turning the mod off also drops a pending burp.
fn burp_click(game: &mut crate::game::Game<'_>, mod_id: &str, button: usize) {
    use crate::game::burp as bp;
    if mod_id != bp::ID {
        return;
    }
    let game = &*game;
    let mut state = bp::state(|key| game.native_setting(bp::ID, key));
    if state.click(button) && !state.on {
        bp::cancel(&bp::PENDING);
    }
    bp::set_state(state);
    for (key, value) in state.stored() {
        game.native_set_setting(bp::ID, key, &value);
    }
}
