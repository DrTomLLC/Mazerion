// TEMPORARILY DISABLED - User DB features removed during encyclopedia integration
// Re-enable after user DB repos are restored

use clap::Parser;

#[derive(Debug, Parser)]
pub enum DbCommand {
    #[command(about = "Database commands temporarily disabled")]
    Disabled,
}

impl DbCommand {
    pub fn execute(&self) -> anyhow::Result<()> {
        println!("Database commands temporarily disabled during encyclopedia integration.");
        println!("Encyclopedia repos are now active. User DB features will be restored later.");
        Ok(())
    }
}