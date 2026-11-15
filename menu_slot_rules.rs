fn scguns_menu_slot_rules(menu: &str, slot: usize, stack: &horizon_client::items::ItemStack, may_place: &mut bool, max_stack: &mut u32) {
    crate::game::port_scguns::menu_slot_rules(menu, slot, stack, may_place, max_stack);
}
