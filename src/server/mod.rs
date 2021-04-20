mod endpoints;
mod error;
mod middlewares;
mod recover;
mod routes;
mod util;

pub mod models;

pub async fn start(state: crate::state::StateHandle, conf: crate::ConfigHandle, db: crate::db::Db) {
    use warp::Filter;

    let app = routes::routes(&state, &conf, &db)
        .recover(recover::recover)
        .with(warp::log("app"));

    warp::serve(app).run(([0, 0, 0, 0], 8081)).await;
}
