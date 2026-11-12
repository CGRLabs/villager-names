# Villager Names 0.2.0 — registry submission sheet

Submit at: https://playhorizon.dev/mods/submit (sign in with GitHub as CGRLabs), choose **new
project** — `villager_names` is not in the registry yet.
**Verified in-game 2026-11-12**: the name draws above a villager only while the crosshair is on
it, goes when you look away, comes back when you look again; four villagers named from the
Lowlands with their trades; a name the player gave kept and shown on hover too.

**What changed in 0.2.0:** names used to hang over every villager all the time. Now a name
appears **only while your crosshair is on the villager** — the way a name-tagged mob behaves in
vanilla. The world stays clean until you ask. Nothing else changed: same 620 names, same ten
regions, same trade in the name, same promise never to touch a name you gave yourself.

## Form fields

| Field | Value |
| --- | --- |
| Kind | Native mod (.hnmod) |
| File | `D:\CGRLabs\Horizon-Ecosystem\prototypes\villager-names\target\villager_names-0.2.0.hnmod` |
| Id | `villager_names` |
| Version | `0.2.0` |
| For Minecraft | `26.3` |
| Name | Villager Names |
| Licence | MIT |
| Authors | CGRLabs |
| Source code | https://github.com/CGRLabs/villager-names |
| Categories | social, adventure |
| Tags | villagers, names, npc, immersion, roleplay |
| Icon | `D:\CGRLabs\Horizon-Ecosystem\prototypes\villager-names\icon.png` **(to be supplied)** |
| Hook points | none declared — the bundle is pinned to Horizon `36f4bc00` |
| Public key | `RWSjKGIJUKKXHvvhxzNN2VPPrXSEr47JXVgGpCwrOOHoneuQjTWWvk1g` (the Hub key, the same one Gyro uses) |
| Signature | `D:\CGRLabs\Horizon-Ecosystem\prototypes\villager-names\target\villager_names-0.2.0.hnmod.minisig` |

## Release facts

- Bundle: **13 051 bytes**, SHA-256 `3c24629641c0a9abc5db4101291fee85d9acd017cdff2b7563f05ad4945090ad`
- Pinned to Horizon `36f4bc00d6fc644f1ad118ddd926a997a034cb48`, horizon-mixins
  `d9c83d7096b7ea5b83678da2efdfd6161f74997e`, mixin schema 6
- Signature: minisign with the Hub key, `villager_names-0.2.0.hnmod.minisig`
- Prebuilt Windows client (for players without a Microsoft account):
  `horizon-client-villagernames-0.2.0-windows-x64.zip`, **23 157 258 bytes**,
  SHA-256 `51e8c41b04934a059e21c663c8880d4a1c8949cd0434f885e0150bd7b9ee2363`;
  inside it `horizon-client-villagernames.exe`, **63 135 232 bytes**,
  SHA-256 `2a642c138480e7158f165a0fd14fd007277c8f30462c7d529d469679f5e98d0a`
- All three assets on GitHub release **v0.2.0** of `CGRLabs/villager-names`
  (https://github.com/CGRLabs/villager-names/releases/tag/v0.2.0); GitHub's sha256 digests
  match the local hashes above
- Installed locally: `horizon-windows-x64\horizon-client-villagernames.exe`, bundle and signature
  in `horizon-windows-x64\horizon-native-mods\`

## What it does

Every villager gets a name from the region it lives in, and keeps the trade it had. You see the
name when you look at them — not before.

**The names.** Ten regions, each with its own given names, family names and a folkloric rare
tier — **620 names in all**: 460 given names and 160 family names.

| Region | Where | Register |
| --- | --- | --- |
| The Lowlands | plains, meadows, deciduous woods | English |
| The Northlands | snowy plains, peaks, taiga | Norse |
| The Hills | windswept hills | Scottish border |
| The Dunes | desert, badlands | Arabic / Maghrebi |
| The Savanna | savanna and plateau | West African |
| The Green Deep | jungle, swamp | South and Southeast Asian |
| The Underhollow | mushroom fields, caves | invented, fungal |
| The Emberdeep | the Nether | invented, ashen |
| The Shore | oceans, rivers, beaches | Celtic / Atlantic |
| The Void | the End | invented, hollow |

Three tiers, as in the Java mod: **common** (most villagers), **uncommon** (one in four) and
**rare** (one in twenty — the names of the region's legends).

**The name shows when you look.** A villager's name appears above its head only while your
crosshair is on it, exactly like a vanilla name-tagged mob. Look away and it is gone; the
villager keeps its name in the world's data either way.

**The trade stays in the name.** Horizon shows a villager's custom name on the trade screen
*instead of* its profession (`trading.rs:675` — `let name = custom.unwrap_or_else(|| …the
profession…)`), so naming a villager would hide its job. This mod puts the trade back:

> **Elsbeth Ashdown, the Fletcher**

**The names never move.** A name comes from the villager's own identity, so the same villager has
the same name in every session and on every machine. A villager the player named with a name tag
is never touched — and its name shows on hover like any other. A villager that finds a job later
gets its trade added to the name it already had.

**No gameplay changes.** No trades, prices, professions or AI are touched — the mod is a layer of
names over the world you already have.

## In-game proof — VERIFIED 2026-11-12

Two harnesses against the real client on scratch worlds. The hover harness cannot trust
`ctl state`'s `name_tags` (that field is computed on the headless serving game, whose HUD is
hidden, so it reads empty even while the window draws the name): it judges the **rendered frame**
instead, from a fixed camera, by pixel difference in the band above the villager's head.

`python tools/proof_hover.py` — one still villager, camera never moves:

| Check | Result |
| --- | --- |
| Crosshair on the villager | its name drawn above it (*Beornwulf Hartwood*) |
| Villager moved aside, camera fixed | name gone — 22 760 pixels changed in the name band |
| Villager moved back | name returns — 23 107 pixels changed against the away frame |
| Name in the entity data while nothing is drawn | kept |
| A name the player gave (*Bartholomew*) | drawn on hover, gone off hover (18 481 pixels), never rewritten |

`python tools/proof_names.py` — a village with a composter, a blast furnace, a brewing stand and
a cartography table:

| Check | Result |
| --- | --- |
| The villagers named | 4, all from the Lowlands table |
| The trades in the names | 4 of 4 — *the Farmer*, *the Armorer*, *the Cleric*, *the Cartographer* |
| The names stayed put | unchanged over 60 ticks |
| A name the player gave | **kept** — the mod did not overwrite it |
| Two villagers with one name | none |

The pictures show it: `target/proof/hover-v02-name.png` (the name over the hovered villager),
`hover-v02-away.png` (same camera, no name), `hover-v02-player-name.png` (*Bartholomew* on hover),
and `names-v02-village.png` (one hovered villager named, the rest of the village clean).

Unit tests, 9 of 9 pass (`cargo test -p horizon-client --lib villager_names::tests`):

| Test | What it holds |
| --- | --- |
| `every_region_has_its_names` | no region is empty or thin |
| `no_region_repeats_a_name` | no name is listed twice |
| `no_biome_is_in_two_regions` | the regions do not overlap |
| `a_name_is_a_given_name_a_family_and_the_trade` | the shape of a name |
| `a_villager_without_a_trade_gets_no_epithet` | a nitwit or an unemployed villager gets no trade |
| `the_same_villager_always_has_the_same_name` | names are stable |
| `a_name_from_an_earlier_session_is_still_ours` | a name the mod gave before is recognised again |
| `a_players_own_name_is_not_ours` | a player's name is never claimed or rewritten |
| `a_rare_name_is_rare` | the rare tier is about one in twenty |

## Description (paste into the Markdown box)

**Everyone in the village has a name.** A villager is a profession in a hat — until it has a
name, and then it is a person who happens to be a farmer. This mod gives every villager a name
from the region it lives in, and keeps the trade it had.

> **Elsbeth Ashdown, the Fletcher** · **Runa Skjold, the Smith** · **Amira Taziri, the Weaver**

**Look at a villager to learn its name.** The name appears above its head only while your
crosshair is on it, the way a name-tagged mob behaves in vanilla — so a village stays a village
to look at, not a wall of floating labels. Look away and the name goes; the villager keeps it.

**The names belong to the place.** The Lowlands, the Northlands, the Dunes, the Green Deep and
six more regions, each with its own names, its own family names and its own legends. A village
in the snow shares family names the way a real village would. Most villagers have ordinary
names; one in four is unusual; one in twenty is a name out of the region's folklore, and worth
stopping for.

**The trade stays in the name.** Naming a villager normally hides its job — the trade screen
shows the name instead of the profession. This mod puts the trade back, so you can tell a
fletcher from a librarian at a glance, and so a villager that finds a job later gets its trade
added to the name it already had.

**Nothing else changes.** No trades, no prices, no professions, no villager AI. It is a layer of
names over the world you already have. A name never moves: the same villager has the same name
in every session and on every machine. And a villager you name yourself with a name tag keeps
that name — this mod will not touch it, and shows it on hover like any other.

*Native mods are unsandboxed code running with your account's permissions; install only from
authors you trust.*

## Still to do (needs the user)

- **The icon** — a villager with a name tag over its head, as the picture shows the mod. PNG, at
  most 256 KiB, dropped at `prototypes/villager-names/icon.png`. The registry square-crops it,
  so a square picture is best.
- **Fill in the form** at https://playhorizon.dev/mods/submit with the table above and the
  description. New project: paste **both** the public key and the signature.
- **Wait for the review.** A project's first listing is always reviewed by a person; later
  versions by the same owner are merged automatically once the checks pass.