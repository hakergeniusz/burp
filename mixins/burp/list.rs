// The mod's row in Options > Mods, with its key.
fn burp_listed(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    use crate::game::burp as bp;
    mods.push(crate::game::native_hooks::NativeMod {
        id: bp::ID.into(),
        title: format!("{} {}", bp::NAME, bp::VERSION),
        description: bp::DESCRIPTION.into(),
        keys: vec![[
            bp::KEY.into(),
            "Burp".into(),
            bp::DEFAULT_KEY.into(),
        ]],
    });
}
