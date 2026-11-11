// Scorched Guns para o Horizon: mod de teste (so valida o caminho GitHub -> launcher -> compilacao).
// Sem comentarios `//!` aqui: modulos injetados nao aceitam atributos internos.

use super::Game;
use std::sync::atomic::{AtomicBool, Ordering};

pub const ID: &str = "scguns";
pub const NAME: &str = "Scorched Guns";
pub const VERSION: &str = "0.0.2";

static LOGGED: AtomicBool = AtomicBool::new(false);

/// Linha de log (vai para latest.log) no primeiro tick do servidor.
pub(super) fn server_tick(_game: &mut Game<'_>) {
    if !LOGGED.swap(true, Ordering::Relaxed) {
        log::info!("scguns: mod carregado, o servidor esta rodando (versao {VERSION})");
    }
}

/// Registra `/scguns hello`.
pub fn register_commands(commands: &mut Vec<super::native_server::NativeCommand>) {
    log::info!("scguns: registrando o comando /scguns hello");
    commands.push(super::native_server::NativeCommand::new("scguns hello"));
}

/// Responde `/scguns hello` no chat de quem digitou.
pub(super) fn perform_command(
    game: &mut Game<'_>,
    player: &super::native_server::NativeServerPlayer,
    command: &str,
    ran: &mut bool,
) {
    let mut words = command.split_whitespace();
    if words.next() == Some("scguns") && words.next() == Some("hello") {
        *ran = true;
        log::info!("scguns: /scguns hello recebido");
        game.native_tell(
            player.uuid,
            horizon_client::text::Text::legacy("[scguns] ola! o mod nativo esta funcionando."),
        );
    }
}

/// A linha do mod em Options > Mods.
pub fn listed(mods: &mut Vec<super::native_hooks::NativeMod>) {
    mods.push(super::native_hooks::NativeMod {
        id: ID.into(),
        title: format!("{NAME} {VERSION}"),
        description: "Port do Scorched Guns 2 (em construcao). Versao de teste.".into(),
        keys: Vec::new(),
    });
}
