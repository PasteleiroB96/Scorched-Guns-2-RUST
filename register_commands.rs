fn scguns_register_commands(commands: &mut Vec<crate::game::native_server::NativeCommand>) {
    crate::game::port_scguns::register_commands(commands);
}
