use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
}

#[allow(dead_code)]
fn _assert_state_traits()
where
    AppState: Clone + Send + Sync + 'static,
{
}
