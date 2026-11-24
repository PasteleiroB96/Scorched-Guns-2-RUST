// Scorched Guns para o Horizon: esqueleto no formato novo (impl Hooks), sem logica ainda.

use mod_api::*;

#[derive(Default)]
struct Mod;

#[mod_api::horizon_mod]
impl Hooks for Mod {
    fn player_logged_in(&self, host: &mut dyn ServerHost, event: &mut PlayerLoggedInEvent<'_>) {
        let _ = (host, event);
    }
}
