fn scguns_menu_slot_rules(menu: &str, slot: usize, stack: &ItemStack, may_place: &mut bool, _max_stack: &mut u32) {
    // O espaco 11 da bancada (o do blueprint) so aceita blueprints do Scorched Guns.
    if menu == "scguns:gun_bench" && slot == 11 {
        let item: &str = &stack.item;
        *may_place = item.starts_with("scguns:") && item.ends_with("_blueprint");
    }
}
