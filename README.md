# Scorched Guns para Horizon (teste de infraestrutura)

Versao 0.0.2. Ainda nao tem armas: so confirma que um mod nativo compila e roda no seu Horizon.

Fixado no Horizon Nightly 2026-11-08, commit `fdc87aecdbd116bf1e08adcd062fcbaea8c2e2ca`.

## O que faz
- Escreve linhas `scguns: ...` no latest.log (primeiro tick do servidor e ao registrar comandos).
- Aparece em Options > Mods.
- Adiciona o comando `/scguns hello`, que responde no chat.

## Como instalar
1. Suba estes arquivos para a RAIZ de um repositorio seu no GitHub (horizon-mixin.toml na raiz).
2. No launcher: Mods > "Adicionar do GitHub" > cole o link do repositorio.
3. Deixe o launcher compilar e jogue.

## Creditos e licenca
Port do mod Scorched Guns 2, de Ribs498 (GPLv3), baseado no Gun Mod de MrCrayfish.
Este port e distribuido sob GPLv3.
