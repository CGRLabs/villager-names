# Villager Names 0.1.0 — registry submission sheet

Submit at: https://playhorizon.dev/mods/submit (sign in with GitHub as CGRLabs), choose **new
project** — `villager_names` is not in the registry yet.
**Verified in-game 2026-11-11**: four villagers named from the Lowlands, all four with their
trade, and a name the player gave kept.

## Form fields

| Field | Value |
| --- | --- |
| Kind | Native mod (.hnmod) |
| File | `D:\CGRLabs\Horizon-Ecosystem\prototypes\villager-names\target\villager_names-0.1.0.hnmod` |
| Id | `villager_names` |
| Version | `0.1.0` |
| For Minecraft | `26.3` |
| Name | Villager Names |
| Licence | MIT |
| Authors | CGRLabs |
| Source code | *(not published yet — see "Still to do")* |
| Categories | social, adventure |
| Tags | villagers, names, npc, immersion, roleplay |
| Icon | `D:\CGRLabs\Horizon-Ecosystem\prototypes\villager-names\icon.png` **(to be supplied)** |
| Hook points | none declared — the bundle is pinned to Horizon `36f4bc00` |
| Public key | `RWSjKGIJUKKXHvvhxzNN2VPPrXSEr47JXVgGpCwrOOHoneuQjTWWvk1g` (the Hub key, the same one Gyro uses) |
| Signature | `D:\CGRLabs\Horizon-Ecosystem\prototypes\villager-names\target\villager_names-0.1.0.hnmod.minisig` |

## Release facts

- File size: **12 610 bytes**
- SHA-256: `a529a41e7383a720d3059d2576fe924195218dbeb112005a4e9e0cc295e34d99`
- Pinned to Horizon `36f4bc00d6fc644f1ad118ddd926a997a034cb48`, horizon-mixins
  `d9c83d7096b7ea5b83678da2efdfd6161f74997e`, mixin schema 6
- `cargo horizon-mod check` → `{"ok":true, …, "kind":"native-mod", "sha256":"a529a41e…34d99"}`
- The mod inspector: **0 errors**, 15 files verified in the bundle
- Signature: minisign, verified against the Hub public key (`signature: VALID`)

## What it does

Every villager gets a name from the region it lives in, and keeps the trade it had.

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

**The trade stays in the name.** Horizon shows a villager's custom name on the trade screen
*instead of* its profession (`trading.rs:675` — `let name = custom.unwrap_or_else(|| …the
profession…)`), so naming a villager would hide its job. This mod puts the trade back:

> **Elsbeth Ashdown, the Fletcher**

**The names never move.** A name comes from the villager's own identity, so the same villager has
the same name in every session and on every machine. A villager the player named with a name tag
is never touched. A villager that finds a job later gets its trade added to the name it already
had.

**No gameplay changes.** No trades, prices, professions or AI are touched — the mod is a layer of
names over the world you already have.

## In-game proof — VERIFIED 2026-11-11

`python tools/proof_names.py` against the real client on a scratch world, with a composter, a
blast furnace, a brewing stand and a cartography table for the villagers to work at.

| Check | Result |
| --- | --- |
| The villagers named | 4, all from the Lowlands table |
| The trades in the names | 4 of 4 — *the Cleric*, *the Armorer*, *the Farmer*, *the Cartographer* |
| The names stayed put | the names did not change over 60 ticks (a trade may be added to a name) |
| A name the player gave | **kept** — the mod did not overwrite it |
| Two villagers with one name | none |

The proof's own picture shows it: *Sewin Barleyleigh, the Cleric* over a villager, and
*Bartholomew* — the name the player gave — untouched beside it.

Unit tests, 7 of 7 pass (`cargo test -p horizon-client --lib villager_names::tests`):

| Test | What it holds |
| --- | --- |
| `every_region_has_its_names` | no region is empty or thin |
| `no_region_repeats_a_name` | no name is listed twice |
| `no_biome_is_in_two_regions` | the regions do not overlap |
| `a_name_is_a_given_name_a_family_and_the_trade` | the shape of a name |
| `a_villager_without_a_trade_gets_no_epithet` | a nitwit or an unemployed villager gets no trade |
| `the_same_villager_always_has_the_same_name` | names are stable |
| `a_rare_name_is_rare` | the rare tier is about one in twenty |

## Description (paste into the Markdown box)

**Everyone in the village has a name.** A villager is a profession in a hat — until it has a
name, and then it is a person who happens to be a farmer. This mod gives every villager a name
from the region it lives in, and keeps the trade it had.

> **Elsbeth Ashdown, the Fletcher** · **Runa Skjold, the Smith** · **Amira Taziri, the Weaver**

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
that name — this mod will not touch it.

*Native mods are unsandboxed code running with your account's permissions; install only from
authors you trust.*

## Still to do (needs the user)

- **The icon** — a villager with a name tag over its head, as the picture shows the mod. PNG, at
  most 256 KiB, dropped at `prototypes/villager-names/icon.png`. The registry square-crops it,
  so a square picture is best.
- **Publish the source** — the registry's pull-request template asks that the `source` link has
  the code the file was built from. The source should go to
  `https://github.com/CGRLabs/villager-names` (a public repo), like Flesh to Leather and Gyro.
- **Fill in the form** at https://playhorizon.dev/mods/submit with the table above and the
  description.
- **Wait for the review.** A project's first listing is always reviewed by a person; later
  versions by the same owner are merged automatically once the checks pass.
