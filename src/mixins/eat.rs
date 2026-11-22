// The local player finished a consumable: when it had food, a big burp is
// on the way. This runs on the single player server's Game (where the use
// completes), so the schedule goes through the shared static and the
// window's tick does the playing. Vanilla's own small burp plays right
// after this hook, inside the function.
// Copyright (c) 2026 hakergeniusz. MIT.
fn burp_eaten(game: &mut crate::game::Game<'_>, info: &horizon_client::items::ItemInfo) {
    // The same gate vanilla's burp uses: food only, no burp for potions.
    if info.food.is_none() {
        return;
    }
    let on = game
        .stored_setting(burp::ID, "on")
        .is_none_or(|v| v.trim() != "false");
    burp::schedule(&burp::PENDING, on);
}
