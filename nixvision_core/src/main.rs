mod ui;
mod commands;
mod io;
mod config;
mod pipelines;
use crate::ui::Interface;
use crate::commands::CommandManager;

fn main() -> opencv::Result<()> {
    // FORMA CORRECTA: Llamada estática (sin instancia)
    Interface::welcome_banner(); 

    if let Err(e) = CommandManager::run_interactive_menu() {
        Interface::error(&format!("Error: {}", e));
    }

    Ok(())
}