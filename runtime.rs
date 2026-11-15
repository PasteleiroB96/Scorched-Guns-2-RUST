// Scorched Guns para o Horizon: bancada (Gun Bench) e base do mod.
// Sem comentarios `//!` aqui: modulos injetados nao aceitam atributos internos.

use super::Game;
use horizon_client::containers::{ModBlockEntity, ModContainer};
use horizon_client::items::ItemStack;
use std::cell::RefCell;
use std::collections::HashSet;
use std::sync::atomic::{AtomicBool, Ordering};

pub const ID: &str = "scguns";
pub const NAME: &str = "Scorched Guns";
pub const VERSION: &str = "0.1.1";

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
        description: "Port do Scorched Guns 2 (em construcao). Gun Bench e armas Antique.".into(),
        keys: Vec::new(),
    });
}

const BENCH: &str = "scguns:gun_bench";
const INPUTS: usize = 10;
const OUTPUT: usize = 10;
const BLUEPRINT: usize = 11;
const BENCH_SLOTS: usize = 12;

/// Onde cada espaco fica na tela (a ordem e a dos espacos do menu): os mesmos pontos do Gun Bench original.
const SLOT_POS: [[f32; 2]; BENCH_SLOTS] = [
    [26.0, 17.0],
    [44.0, 17.0],
    [62.0, 17.0],
    [80.0, 17.0],
    [26.0, 35.0],
    [44.0, 35.0],
    [62.0, 35.0],
    [80.0, 35.0],
    [26.0, 53.0],
    [62.0, 53.0],
    [140.0, 44.0],
    [116.0, 17.0],
];

#[derive(Clone, Copy)]
enum Need {
    Empty,
    Item(&'static str),
    Planks,
}

struct BenchRecipe {
    result: &'static str,
    count: u32,
    blueprint: &'static str,
    needs: [Need; INPUTS],
}

/// Gerado das receitas do mod original (scguns:gun_bench): cada espaco pede exatamente o que a receita diz.
static RECIPES: &[BenchRecipe] = &[
    BenchRecipe { result: "scguns:musket", count: 1, blueprint: "scguns:antique_blueprint", needs: [Need::Empty, Need::Empty, Need::Empty, Need::Empty, Need::Item("minecraft:flint"), Need::Item("minecraft:cobblestone"), Need::Item("scguns:stone_gun_barrel"), Need::Item("scguns:stone_gun_barrel"), Need::Planks, Need::Planks] },
    BenchRecipe { result: "scguns:flintlock_pistol", count: 1, blueprint: "scguns:antique_blueprint", needs: [Need::Empty, Need::Empty, Need::Empty, Need::Empty, Need::Item("minecraft:flint"), Need::Item("minecraft:cobblestone"), Need::Item("scguns:stone_gun_barrel"), Need::Empty, Need::Planks, Need::Planks] },
    BenchRecipe { result: "scguns:handcannon", count: 1, blueprint: "scguns:antique_blueprint", needs: [Need::Empty, Need::Empty, Need::Item("minecraft:stone"), Need::Empty, Need::Item("minecraft:flint"), Need::Item("minecraft:cobblestone"), Need::Item("scguns:stone_gun_barrel"), Need::Empty, Need::Planks, Need::Planks] },
    BenchRecipe { result: "scguns:blunderbuss", count: 1, blueprint: "scguns:antique_blueprint", needs: [Need::Empty, Need::Empty, Need::Item("minecraft:stone"), Need::Empty, Need::Item("minecraft:flint"), Need::Item("minecraft:cobblestone"), Need::Item("scguns:stone_gun_barrel"), Need::Item("scguns:stone_gun_barrel"), Need::Planks, Need::Planks] },
    BenchRecipe { result: "scguns:doublet", count: 1, blueprint: "scguns:antique_blueprint", needs: [Need::Empty, Need::Empty, Need::Item("minecraft:stone"), Need::Item("scguns:stone_gun_barrel"), Need::Item("minecraft:flint"), Need::Item("minecraft:cobblestone"), Need::Item("minecraft:stone"), Need::Item("scguns:stone_gun_barrel"), Need::Planks, Need::Planks] },
];

thread_local! {
    static ARMED: RefCell<HashSet<[i32; 3]>> = RefCell::new(HashSet::new());
}

fn name(slot: &Option<ItemStack>) -> Option<&str> {
    match slot {
        Some(s) if s.count > 0 => Some(&*s.item),
        _ => None,
    }
}

fn fits(need: Need, have: Option<&str>) -> bool {
    match (need, have) {
        (Need::Empty, None) => true,
        (Need::Item(want), Some(got)) => want == got,
        (Need::Planks, Some(got)) => got.starts_with("minecraft:") && got.ends_with("_planks"),
        _ => false,
    }
}

fn find(items: &[Option<ItemStack>]) -> Option<&'static BenchRecipe> {
    if items.len() < BENCH_SLOTS {
        return None;
    }
    let blueprint = name(&items[BLUEPRINT])?;
    RECIPES
        .iter()
        .find(|r| r.blueprint == blueprint && (0..INPUTS).all(|i| fits(r.needs[i], name(&items[i]))))
}

fn take_one(slot: &mut Option<ItemStack>) {
    let empty = match slot.as_mut() {
        Some(s) => {
            s.count = s.count.saturating_sub(1);
            s.count == 0
        }
        None => false,
    };
    if empty {
        *slot = None;
    }
}

/// `native_block_entity`: a bancada tem um block entity proprio.
pub fn block_entity(block: &str, entity: &mut Option<(String, horizon_world::nbt::Compound)>) {
    if entity.is_none() && block == BENCH {
        *entity = Some((block.to_owned(), horizon_world::nbt::Compound::new()));
    }
}

pub fn block_entity_ticks(block: &str, ticks: &mut bool) {
    if block == BENCH {
        *ticks = true;
    }
}

/// `native_block_entity_container`: 12 espacos e o menu scguns:gun_bench (sem funil: so a mao).
pub fn block_entity_container(block: &str, out: &mut Option<ModContainer>) {
    if block == BENCH {
        *out = Some(ModContainer {
            slots: BENCH_SLOTS,
            faces: std::array::from_fn(|_| Vec::new()),
            inputs: Vec::new(),
            outputs: Vec::new(),
            menu: Some(BENCH.to_owned()),
        });
    }
}

/// A cada tick do servidor: se os espacos batem com uma receita, poe a arma no resultado; quando o jogador a
/// retira, gasta um de cada ingrediente (o blueprint fica).
pub(super) fn block_entity_tick(_game: &mut Game<'_>, pos: [i32; 3], entity: &mut ModBlockEntity) {
    if &*entity.id != BENCH || entity.items.len() < BENCH_SLOTS {
        return;
    }
    let matched = find(&entity.items);
    let has_output = entity.items[OUTPUT].as_ref().map_or(false, |s| s.count > 0);
    if has_output {
        let still_valid = match (matched, entity.items[OUTPUT].as_ref()) {
            (Some(r), Some(s)) => &*s.item == r.result,
            _ => false,
        };
        if still_valid {
            ARMED.with_borrow_mut(|a| {
                a.insert(pos);
            });
        } else {
            entity.items[OUTPUT] = None;
            ARMED.with_borrow_mut(|a| {
                a.remove(&pos);
            });
        }
        return;
    }
    let armed = ARMED.with_borrow_mut(|a| a.remove(&pos));
    if armed {
        if let Some(r) = matched {
            for i in 0..INPUTS {
                if !matches!(r.needs[i], Need::Empty) {
                    take_one(&mut entity.items[i]);
                }
            }
        }
    }
    if let Some(r) = find(&entity.items) {
        entity.items[OUTPUT] = Some(ItemStack::new(r.result, r.count));
        ARMED.with_borrow_mut(|a| {
            a.insert(pos);
        });
    }
}

/// `native_screen`: a tela da bancada, 176 x 166, com os espacos do original.
pub fn screen(_game: &Game<'_>, block: &str, out: &mut Option<super::native_hooks::NativeScreen>) {
    use super::native_hooks::NativeScreen;
    if block == BENCH {
        *out = Some(NativeScreen {
            width: 176.0,
            height: 166.0,
            slots: SLOT_POS.iter().map(|p| Some(*p)).collect(),
            inventory_y: 84.0,
            inventory_x: 8.0,
            own_labels: false,
            own_background: false,
        });
    }
}

/// `native_screen_draw`: o fundo (assets/scguns/textures/gui/gun_bench_gui.png).
pub fn screen_draw(game: &Game<'_>, block: &str, batch: &mut horizon_client::hud::HudBatch, origin: [f32; 2]) {
    if block == BENCH {
        game.native_sprite(
            batch,
            "scguns:gui/gun_bench_gui",
            [0.0, 0.0, 176.0, 166.0],
            [origin[0], origin[1], 176.0, 166.0],
        );
    }
}
