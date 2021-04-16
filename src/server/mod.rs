mod endpoints;
mod error;
mod middlewares;
pub mod models;

pub async fn start(state: crate::state::StateHandle) {
    // Start server
    let mut app = tide::with_state(state.clone());
    app.middleware(middlewares::not_found);

    // Index
    app.at("/").get(|_req| async { "Market API Server" });

    // Resources
    app.at("/resEditor").get(endpoints::get_resource_editor);
    app.at("/res/:key").get(endpoints::get_resource);
    app.at("/res").nest({
        let mut authed_router = tide::with_state(state);
        authed_router.middleware(middlewares::authenticate);

        authed_router.at("").get(endpoints::get_all_resources);
        authed_router.at("").post(endpoints::post_resource);
        authed_router.at("/:key").delete(endpoints::delete_resource);

        authed_router
    });

    // All items
    app.at("/all").get(endpoints::get_all_endpoint);

    // Started
    log::info!("Server started!");
    app.listen("0.0.0.0:8080").await.unwrap();
}
