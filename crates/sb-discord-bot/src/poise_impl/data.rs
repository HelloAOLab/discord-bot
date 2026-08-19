pub struct Data {
    pub store: std::sync::Arc<dyn crate::store::store::Store>,
    pub scheduler: tokio_cron_scheduler::JobScheduler,
}
