mod endpoints;
mod error;
mod middlewares;
mod recover;
mod routes;
mod util;

pub mod models;

pub async fn start(state: crate::state::StateHandle, conf: crate::ConfigHandle) {
    use warp::Filter;

    let app = routes::routes(&state, &conf)
        .recover(recover::recover)
        .with(warp::log("app"));

    warp::serve(app).run(([0, 0, 0, 0], 8081)).await;
}
