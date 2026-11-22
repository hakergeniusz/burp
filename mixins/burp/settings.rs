// The mod's settings page: the on/off toggle.
fn burp_page(game: &mut crate::game::Game<'_>, mod_id: &str, buttons: &mut Vec<String>) {
    use crate::game::burp as bp;
    if mod_id != bp::ID {
        return;
    }
    let game = &*game;
    let state = bp::state(|key| game.native_setting(bp::ID, key));
    buttons.extend(state.labels());
}
