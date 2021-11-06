use sqlx::PgPool;
use std::sync::Arc;

use caster_shows::shows_service::{PgShowsService, ShowsService};

/// The Services for this App
pub struct Services {
    shows: Arc<dyn ShowsService>,
}

impl Services {
    /// Create a new instance of the app Services with a Postgres Pool
    pub fn new(pg_pool: PgPool) -> Self {
        Self {
            shows: Arc::new(PgShowsService::new(pg_pool)),
        }
    }
}
