mod endpoints;
mod error;
mod middlewares;
mod recover;
mod routes;
mod util;

pub mod models;

pub async fn init(
    conf: crate::ConfigHandle,
    db: crate::db::Db,
) -> impl warp::Filter<Extract = impl warp::Reply, Error = std::convert::Infallible> + Clone {
    use warp::Filter;

    routes::routes(&conf, &db)
        .recover(recover::recover)
        .with(warp::log("app"))
}
