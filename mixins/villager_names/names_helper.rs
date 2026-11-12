// The server half: name the villagers, and keep their names right.
//
// It runs where the world is simulated — single player and a LAN host. A
// client that joins someone else's server sees the names the server gave.

fn on_level_tick_post(game: &mut crate::game::Game<'_>, dimension: &str) {
    use horizon_client::villager_names as vn;

    let _ = dimension;

    // The villagers and where they stand, read before the world is changed.
    let villagers: Vec<(u32, [f64; 3], Option<String>, &'static str)> = game
        .mobs
        .iter()
        .filter(|m| vn::wants_a_name(m))
        .map(|m| {
            (
                m.id,
                [m.position.x, m.position.y, m.position.z],
                m.custom_name.clone(),
                vn::profession_of(m),
            )
        })
        .collect();
    // The villagers that are still here, so the names of the ones that are not
    // can be forgotten.
    let living: std::collections::HashSet<u32> = game.mobs.iter().map(|m| m.id).collect();
    vn::forget_gone(&living);

    for (id, pos, current, profession) in villagers {
        // The biome the villager stands in, as the game reads it.
        let (x, y, z) = (
            pos[0].floor() as i32,
            pos[1].floor() as i32,
            pos[2].floor() as i32,
        );
        let biome = game
            .world
            .chunks
            .get(&(x >> 4, z >> 4))
            .map_or_else(|| "minecraft:plains".to_owned(), |c| {
                game.res.biomes().list[c.biome(x, y, z) as usize]
                    .name
                    .to_string()
            });
        let region = vn::region_of(&biome).unwrap_or(vn::REGIONS[0]);
        let name = vn::name_for(region, id, profession);
        let Some(m) = game.mobs.iter_mut().find(|m| m.id == id) else {
            continue;
        };
        match current {
            // A name we gave — this session or an earlier one: keep it, bring
            // the trade up to date, and show it only under the crosshair.
            Some(current)
                if vn::is_ours(id, &current)
                    || vn::named_by_us(region, id, profession, &current) =>
            {
                if name != current {
                    m.custom_name = Some(name.clone());
                    m.needs_sync = true;
                }
                if m.name_visible {
                    m.name_visible = false;
                    m.needs_sync = true;
                }
                vn::remember(id, &name);
            }
            // A name the player gave with a name tag: never touched.
            Some(_) => {}
            None => {
                m.custom_name = Some(name.clone());
                // Hover-only: game/names.rs shows a name when name_visible or
                // the mob is under the crosshair. We want the latter.
                m.name_visible = false;
                // The window client only learns of a name change when the mob
                // is marked for sync (game/native_mobs.rs does the same).
                m.needs_sync = true;
                vn::remember(id, &name);
            }
        }
    }
}
