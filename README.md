# Villager Names

Every villager gets a name from the region it lives in — and keeps the trade it had.

A village of eight no longer stops being a village the moment you name one of them. With this
mod, every villager has a name, the names belong to the place, and you can still tell a
fletcher from a librarian at a glance.

> **Elsbeth Ashdown, the Fletcher** · **Runa Skjold, the Smith** · **Amira Taziri, the Weaver**

## What it does

- **Names every villager**, from the region it lives in — the Lowlands, the Northlands, the
  Dunes, the Green Deep and six more, each with its own names and its own family names.
- **Keeps the trade in the name.** Horizon shows a villager's custom name on the trade screen
  *instead of* its profession, so a named villager would lose its job. This mod puts the trade
  back: *Elsbeth Ashdown, the Fletcher*.
- **Never changes a name.** A villager's name is derived from who it is, so the same villager has
  the same name in every session and on every machine. A villager you named with a name tag keeps
  that name; the mod never touches it.
- **Touches no gameplay.** No trades change, no prices change, no villager AI changes. It is a
  layer of names over the world you already have.

## The names

| Tier | How often | What they are |
| --- | --- | --- |
| common | most villagers | the everyday names of the region |
| uncommon | one in four | rarer, more characterful |
| rare | one in twenty | folkloric — the names of the region's legends |

Each region has its own families too, so a village of Northlanders shares family names the way a
real village would.

## Install

1. Install the mod and enable it in Options → Mods, then restart the game.
2. Walk into a village.

It works in single player and on a LAN host — the places where the world is simulated by a
machine that has the mod. Joining someone else's server, you see the names that server gave; the
mod cannot add its own there.

## Build from source

```sh
HORIZON_MIXINS=/path/to/horizon-mixins \
  python tools/horizon.py build --horizon /path/to/horizon
```

The names live in `mixins/villager_names/names_*.rs` — one file per region, so a region can be
re-written without touching the rest.

## The icon

Drop it at `icon.png` in this folder. The registry takes a **PNG of at most 256 KiB**; the
listing square-crops it, so a square picture is best.

> A villager with a name tag over its head — the name is the point of the mod, so it should be
> the first thing you see.

## Licence

[MIT](LICENSE) — Copyright (c) 2026 CGRLabs.

Not an official Minecraft product. Not approved by or associated with Mojang or Microsoft.
