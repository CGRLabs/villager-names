# Villager Names — WebAssembly `.hmod`

This directory is the 0.3.0 port of Villager Names to Horizon's sandboxed mod API. The old
`.hnmod` mixin tree at the repository root remains for history; this is the installable build.

- **id:** `villager_names`
- **kind:** WebAssembly (`mod`, side `both`)
- **SDK:** `0b14f8acfd97fa2c750e42795249dcc981178ca4`
- **Toolchain:** Rust 1.99.1

## Behaviour

Every adult villager gets a one-word name from the region it lives in — the Lowlands, the
Northlands, the Hills, the Dunes, the Savanna, the Greendeep, the Underhollow, the Emberdeep, the
Shore, the Void. The name is drawn from the villager's own entity id, so the same villager always
has the same name: in this session, in the next one, and on someone else's machine.

A name shows only while the crosshair is on the villager, as a vanilla name tag does — not hanging
over every villager in the village.

A name the player gave with a name tag is never touched. A name from an older version of this mod
(`Elsbeth Ashdown, the Fletcher`) is recognised as ours and shortened to the one word.

Babies are left unnamed.

## Why the port is not a straight translation

The retired native build reached into `Game`'s mob list and had to flag every renamed mob with
`needs_sync` by hand; 0.1.x shipped without that flag and names never reached the window client.
`EntityRef::set_custom_name` sends the change to the players tracking the mob itself, so the
sandboxed version cannot make that mistake: there is no flag to forget.

The region is read with `LevelRef::biome_at`, and the name with `EntityRef::name` — no private
field access, so no mixins, and the whole mod is reviewable as a wasm guest.

## Rules and tests

`src/names.rs` holds the ten region tables, carried over verbatim from the native build.
`src/rules.rs` is the pure logic: which region a biome belongs to, the name an id picks, and
whether a name already on a villager is ours, a player's, or nothing at all. `src/server.rs` is the
thin adapter to Horizon handles.

```
cargo test            # 14 tests, host target
```

## Build and check

```sh
cargo horizon-mod build --wasm
cargo horizon-mod check target/horizon-mod/villager_names.hmod
```

## Licence

MIT. The licence text ships inside the package as `LICENSE`.
