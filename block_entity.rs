fn scguns_block_entity(block: &str, entity: &mut Option<(String, horizon_world::nbt::Compound)>) {
    crate::game::port_scguns::block_entity(block, entity);
}
