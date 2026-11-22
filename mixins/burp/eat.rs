// The local player finished a consumable: when it had food, a big burp is
// on the way. This runs on the single player server's Game (where the use
// completes), so the schedule goes through the shared static and the
// window's tick does the playing. Vanilla's own small burp plays right
// after this hook, inside the function.
fn burp_eaten(game: &mut crate::game::Game<'_>, info: &horizon_client::items::ItemInfo) {
    use crate::game::burp as bp;
    // The same gate vanilla's burp uses: food only, no burp for potions.
    if info.food.is_none() {
        return;
    }
    // Turned off in Options > Mods: nothing until the next build leaves it out.
    if !game.native_on(bp::ID) {
        return;
    }
    bp::schedule(&bp::PENDING, true);
}
