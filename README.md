# Burp Mod (Horizon native mod)

A native mod for **Horizon** (Minecraft Java Edition 26.3 in Rust) that lets
you burp on demand and burps big a beat after finishing a meal.

- **Burp key** — press **B** (rebindable in *Options > Mods*) to burp with a
  random pitch (0.85–1.2) — the player's own recording, like the big burp.
- **After-meal burp** — finish eating any food and, 12 ticks (0.6 s) later,
  a **big burp** plays: the player's own recording (trimmed from a YouTube
  sound-effect video), registered as the mod-namespaced event
  `burpmod:big_burp` at a deep 0.55–0.75 pitch. Vanilla's own small burp
  still plays at the meal — this is the big one on top. Potions don't count
  (no food component), interrupted bites don't count.
- **Settings** — *Options > Mods > Burp Mod*: an ON/OFF toggle, persisted in
  `horizon-mods/.horizon-mod-data/burp/on`; turning it off drops a pending
  burp.

## How it works (the interesting bit)

Single player runs **two Games**: the window's and the single player
server's — and `native_tick` fires in **both**. The pieces end up on
different sides, measured live with instrumented hooks:

- `Game::finish_consuming` (the eat) runs on the **server's Game**, which
  does have a server-side player — that's where vanilla's own small burp is
  played from.
- The eat hook therefore schedules the big burp into a **static
  `AtomicU32`** (`PENDING`), not thread-local state: thread-locals are
  per-Game and would never be seen by the other side.
- The tick hook runs in both Games but **fires only on `native_window()`**,
  the Game with the local player's position and audio — so the burp is one
  proper local sound, not a server-side `None`-position play.

The custom sound lives in the prepared resources:
`.local/horizon-client/26.3/assets/burpmod/sounds.json` +
`sounds/big_burp.ogg` — the client's pack scan merges any namespace's
`sounds.json`, no code needed for the registration itself.

## Files

```
Cargo.toml                  the mod's library (runtime.rs) — its tests run standalone
compatibility.json          pinned Horizon + horizon-mixins revisions
mixins/burp/horizon-mixin.toml   the bundle: one module + six HEAD injections
mixins/burp/runtime.rs      state, pitches, the shared PENDING atomic — pure logic + tests
mixins/burp/key.rs          native_key HEAD: the burp key (window Game)
mixins/burp/tick.rs         native_tick HEAD: fires the pending big burp (window Game only)
mixins/burp/eat.rs          Game::finish_consuming HEAD: schedules (server Game)
mixins/burp/list.rs         native_mods HEAD: the Options > Mods row
mixins/burp/settings.rs     native_settings HEAD: the settings page
mixins/burp/click.rs        native_settings_click HEAD: the toggle (+ cancel pending)
tools/build.sh              tests → apply mixins → build client → install
```

## Building

```zsh
tools/build.sh              # tests + build + install (~6 min)
tools/build.sh --skip-tests # build + install only
```

The script pins the launcher's Rust toolchain and applies the mixins to the
pristine source at
`~/.local/share/horizon-launcher/native/source/af55a734.../`, builds the
client (`fast` profile) in `~/.cache/horizon-burp-build` and installs it as
`~/Downloads/horizon-linux-x86_64/horizon-client-burp` via temp+mv (never
touching a running binary — the stock client and other variants are safe).

## Playing

Start it from the bundle folder so it finds the prepared resources:

```zsh
cd ~/Downloads/horizon-linux-x86_64 && ./horizon-client-burp
```

## Verification (2026-11-08, revision af55a734)

Headless test under Xvfb with horizon-ctl (`key b tap`, `give bread`,
`use-item --hold 5`); eating needs food < 20 — fall damage + regen drains it
(`tp x y+12 z` on dry land; falling into water does nothing, and the hunger
status effect doesn't drain food at all). Give the world ~30 s after launch
before item use works. Final run:

- `heard` after two key presses: `burpmod:big_burp: 2`, vanilla burp event
  unused by the mod (`0`). 10/10 unit tests. One meal also verified earlier:
  `burpmod:big_burp: 1` + `minecraft:entity.player.burp: 1` (vanilla's own
  small burp) with log lines `burp: big burp after the meal (pitch 0.74)`,
  `burp: key burp (pitch 1.14)`.

## MIT licensed
