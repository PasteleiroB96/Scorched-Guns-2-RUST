// Scorched Guns para o Horizon: mod de teste (so valida o caminho GitHub -> launcher -> compilacao).
// Sem comentarios `//!` aqui: modulos injetados nao aceitam atributos internos.

use super::Game;
use std::sync::atomic::{AtomicBool, Ordering};

pub const ID: &str = "scguns";

static LOGGED: AtomicBool = AtomicBool::new(false);

/// Escreve uma linha no log na primeira vez que o servidor faz um tick (aba Console do launcher).
pub fn server_tick(_game: &mut Game<'_>) {
    if !LOGGED.swap(true, Ordering::Relaxed) {
        eprintln!("[{ID}] mod carregado: o servidor esta rodando");
    }
}

/// Registra `/scguns hello`.
pub fn register_commands(commands: &mut Vec<super::native_server::NativeCommand>) {
    commands.push(super::native_server::NativeCommand::new("scguns hello"));
}

/// Responde `/scguns hello` no chat de quem digitou.
pub fn perform_command(
    game: &mut Game<'_>,
    player: &super::native_server::NativeServerPlayer,
    command: &str,
    ran: &mut bool,
) {
    let mut words = command.split_whitespace();
    if words.next() == Some("scguns") && words.next() == Some("hello") {
        *ran = true;
        game.native_tell(
            player.uuid,
            horizon_client::text::Text::legacy("[scguns] ola! o mod nativo esta funcionando."),
        );
    }
}
