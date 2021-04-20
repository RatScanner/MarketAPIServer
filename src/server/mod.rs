mod endpoints;
mod error;
mod middlewares;
mod recover;
mod routes;
mod util;

pub mod models;

pub async fn init(
    state: crate::state::StateHandle,
    conf: crate::ConfigHandle,
    db: crate::db::Db,
) -> impl warp::Filter<Extract = impl warp::Reply, Error = std::convert::Infallible> + Clone {
    use warp::Filter;

    routes::routes(&state, &conf, &db)
        .recover(recover::recover)
        .with(warp::log("app"))
}
