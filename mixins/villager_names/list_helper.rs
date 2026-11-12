fn on_mods(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    use horizon_client::villager_names as vn;
    mods.push(crate::game::native_hooks::NativeMod {
        id: vn::ID.into(),
        title: format!("Villager Names {}", vn::VERSION),
        description: vn::DESCRIPTION.into(),
        keys: vec![],
    });
}
