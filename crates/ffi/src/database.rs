// TEMPORARILY DISABLED - User DB features removed during encyclopedia integration
// Re-enable after user DB repos are restored

use std::sync::Arc;
use mazerion_db::DatabaseManager;

pub struct MazerionDatabase {
    manager: Arc<DatabaseManager>,
}

impl MazerionDatabase {
    pub fn new(manager: Arc<DatabaseManager>) -> Self {
        Self { manager }
    }
    
    // Encyclopedia access methods will be added here
    // User database functionality temporarily disabled
}