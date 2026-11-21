# Villager Names 0.3.0 — registry submission sheet

Submit at: https://playhorizon.dev/mods/submit (sign in with GitHub as CGRLabs), choose **new
project** — `villager_names` is not in the registry yet.

**Verified in-game 2026-11-21** on a client built from the pinned Horizon `0b14f8ac`: eight
villagers were each given a one-word Lowlands name, the names were unchanged after 60 ticks, and a
name the player gave was kept.

**What changed in 0.3.0:** the mod is now a sandboxed WebAssembly package (`.hmod`) instead of a
native `.hnmod` bundle. Same 620 names, same ten regions, same rule that a name never changes once
given. It also fixes the bug that sank 0.1.x — see *Why the port* below.

## Form fields

| Field | Value |
| --- | --- |
| Kind | WebAssembly mod (`.hmod`) |
| File | `D:\CGRLabs\Horizon-Ecosystem\.workbuddy-ai\worktrees\villager-names-wasm\hmod\target\horizon-mod\villager_names.hmod` |
| Id | `villager_names` |
| Version | `0.3.0` |
| For Minecraft | `26.3` |
| Name | Villager Names |
| Licence | MIT |
| Authors | CGRLabs |
| Source code | https://github.com/CGRLabs/villager-names — branch `hmod/villager-names-0.3.0` |
| Categories | social, adventure |
| Tags | villagers, names, npc, immersion, roleplay |
| Icon | **you are supplying it** — must be a 256×256 PNG, at most 256 KiB |
| Hook points | none — the package declares no `[[inject]]` listeners |
| Public key | `RWSjKGIJUKKXHvvhxzNN2VPPrXSEr47JXVgGpCwrOOHoneuQjTWWvk1g` (the Hub key, the same one Gyro and Day Counter use) |

## Package facts

- `villager_names.hmod`: **2 676 611 bytes**, SHA-256
  `37d26dfc9363f7dce53613b367715cb4614da587b5b1908ab17bac4e5a7ec685`
- Manifest: `api = "0.2"`, `side = "both"`, `capabilities = ["registries"]`. That capability list
  is required on every `.hmod` whatever the mod registers — this one registers nothing — because the
  package manifest is the content-side one and that schema rejects anything else.
- `[wasm]`: `sha512` of `mod.wasm` `7ed85ba0a52c5231f63aeb5c05cf8bb46fb943218c6f9b4d41b31d16c1a12c7753de40eb557686061f4f9a99116d08831f4731a1df8e647be27f8ba26e687cb7`,
  `toolchain = "1.99.1 58cf8f9c35b355199727627d07ae34a4e2846d67"`. The tool wrote both; neither is
  hand-edited.
- Built with `cargo horizon-mod build --wasm` from the pinned SDK (`0b14f8acfd97fa2c750e42795249dcc981178ca4`),
  which also validates the module as a shared group member (`dylink.0`, and the `horizon_layout` /
  `horizon_in` / `horizon_hook` exports).
- A game that runs this needs `horizon-wasm-std.wasm` beside its executable; releases ship it. It is
  not part of the package and not something a submitter supplies.

## What it does

Every villager gets a name from the region it lives in, and keeps the trade it had. The name draws
only while your crosshair is on that villager — the world stays clean until you ask, the way a
name-tagged mob behaves in vanilla.

**The names.** Ten regions, each with its own given names, family names and a folkloric rare tier —
**620 names in all**. The name is a function of the villager's entity id and its region, so nothing
has to be remembered between ticks and a name cannot drift.

**A name you give is never touched.** The mod names a villager only when it has no name of its own,
or when the name on it is one the mod would have given. Anything else is left alone, including a
player's own tag.

## Why the port

The native build set the custom name on the serving game and had to flag `needs_sync` by hand. 0.1.x
shipped without that flag, so names never reached the window client. In a sandboxed build the mod
calls `EntityRef::set_custom_name`, which sends the change to the players tracking that mob itself —
a port cannot repeat the omission.

## In-game proof — VERIFIED 2026-11-21

Run against a client built from the pin, with the package installed in `horizon-mods/`:

```
python tools/proof_names.py --game <horizon-client-wasm.exe> --ctl <horizon-ctl-wasm.exe> \
    --resources <prepared resources> --attach --home <game home>
```

The run builds a stone floor with four workstations, summons villagers, and then checks:

| Check | Result |
| --- | --- |
| Villagers named | 8 of 8 adults |
| The names | `Alwin`, `Beornwulf`, `Sewin`, `Wendelin`, `Thurstan`, `Eadweard`, `Gytha`, `Baldwin` |
| Every name is one word, from the Lowlands table | passed |
| Names do not move over 60 ticks | unchanged |
| A name the player gives is kept | `Bartholomew` survived |

`"passed": true` in `target/proof/names-v03-wasm-report.json`; the run log is
`target/proof/names-v03-wasm.log`, and `target/proof/names-v03-wasm-village.png` shows the name
drawn above the villager under the crosshair.

Unit tests: `cargo test` in `hmod/` — 15 tests covering the region table, the name a given id
picks, and the decision to name or leave a villager.

## Still to do before the form

1. **The icon** — you are supplying it (256×256 PNG).
2. **The signature.** No `.minisig` exists for these bytes: `minisign` is not installed on this
   machine, so signing could not be done here. The 0.2.0 bundle was signed with the Hub key above.
   Confirm at submit time whether the wasm submission asks for a signature; if it does, sign
   `villager_names.hmod` with the Hub key and record the digest here.
3. **Nothing else.** The package builds, validates, and is proven in-game on the exact bytes whose
   SHA-256 is written above.
