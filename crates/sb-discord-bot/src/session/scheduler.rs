use std::{sync::Arc, time::Duration};

use serenity::all::Http;
use tokio_cron_scheduler::{Job, JobScheduler, JobSchedulerError};

use crate::{
    session::jobs,
    store::store::{ScheduledSession, Store},
};

fn delay_from_now(target_unix: i64, now_unix: i64) -> Duration {
    Duration::from_secs((target_unix - now_unix).max(0) as u64)
}

/// Registers the start job (and, if configured, a reminder job) for a single
/// scheduled session. Called both right after `/launchsession` creates a new
/// row, and once per pending row at process startup — the relative delay is
/// always recomputed from the stored absolute `scheduled_at`, so a bot
/// restart after the scheduled time just fires it immediately instead of
/// dropping it.
pub async fn register_session_jobs(
    sched: &JobScheduler,
    http: Arc<Http>,
    store: Arc<dyn Store>,
    session: &ScheduledSession,
    now_unix: i64,
) -> Result<(), JobSchedulerError> {
    let start_delay = delay_from_now(session.scheduled_at, now_unix);
    let (job_http, job_store, session_id) = (Arc::clone(&http), Arc::clone(&store), session.id);
    sched
        .add(Job::new_one_shot_async(
            start_delay,
            move |_uuid, _sched| {
                let (job_http, job_store) = (Arc::clone(&job_http), Arc::clone(&job_store));
                Box::pin(async move { jobs::run_start_job(job_http, job_store, session_id).await })
            },
        )?)
        .await?;

    if let (Some(minutes), false) = (session.reminder_minutes, session.reminder_sent) {
        let reminder_at = session.scheduled_at - minutes * 60;
        if reminder_at < session.scheduled_at {
            let reminder_delay = delay_from_now(reminder_at, now_unix);
            let (job_http, job_store, session_id) =
                (Arc::clone(&http), Arc::clone(&store), session.id);
            sched
                .add(Job::new_one_shot_async(
                    reminder_delay,
                    move |_uuid, _sched| {
                        let (job_http, job_store) = (Arc::clone(&job_http), Arc::clone(&job_store));
                        Box::pin(async move {
                            jobs::run_reminder_job(job_http, job_store, session_id).await
                        })
                    },
                )?)
                .await?;
        }
    }

    Ok(())
}

/// Re-registers every still-pending session's jobs on startup, so scheduled
/// sessions survive a bot restart.
pub async fn bootstrap_pending_sessions(
    sched: &JobScheduler,
    http: Arc<Http>,
    store: Arc<dyn Store>,
) -> Result<(), JobSchedulerError> {
    let now_unix = chrono::Utc::now().timestamp();
    for session in store.list_pending_sessions().await {
        register_session_jobs(
            sched,
            Arc::clone(&http),
            Arc::clone(&store),
            &session,
            now_unix,
        )
        .await?;
    }
    Ok(())
}
