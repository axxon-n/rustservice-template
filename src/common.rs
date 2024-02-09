use std::sync::Arc;
use tokio::sync::Semaphore;

pub struct AppState {
    //pub db_svc: ScyllaService,
    pub semaphore: Arc<Semaphore>,
    pub is_test: bool
}