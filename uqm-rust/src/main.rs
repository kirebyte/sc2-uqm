//! The Ur-Quan Masters - Main Entry Point
//!
//! This is a modern Rust reimplementation of The Ur-Quan Masters (Star Control 2).

use uqm::GameState;

fn main() {
    // Initialize logging
    env_logger::init();
    log::info!("Starting UQM Rust v{}", uqm::VERSION);

    // Create a new game state
    let game_state = GameState::new();
    
    log::info!("Game initialized at Sol system");
    log::info!("Fuel: {}", game_state.fuel);
    log::info!("Crew: {}", game_state.crew);
    log::info!("Time: {}", game_state.time.format());

    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║      THE UR-QUAN MASTERS - RUST REIMPLEMENTATION         ║");
    println!("║                    Version {}                        ║", uqm::VERSION);
    println!("╚════════════════════════════════════════════════════════════╝");
    println!();
    println!("Phase 0: Project Setup Complete");
    println!();
    println!("Game initialized successfully:");
    println!("  Location: Sol System ({}, {})", game_state.location.x, game_state.location.y);
    println!("  Fuel:     {} units", game_state.fuel);
    println!("  Crew:     {} members", game_state.crew);
    println!("  Date:     {}", game_state.time.format());
    println!();
    println!("Next Steps:");
    println!("  - Phase 1: Core Types & Structures");
    println!("  - Phase 2: Graphics Layer");
    println!("  - Phase 3-8: Full game implementation");
    println!();
    println!("Press Ctrl+C to exit");

    // TODO: Game loop will be implemented in later phases
    // For now, this is just a proof of concept
}
