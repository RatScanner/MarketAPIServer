use super::{endpoints, middlewares, util::PercentDecoded};
use crate::{db::Db, state::StateHandle, ConfigHandle};
use warp::{Filter, Rejection, Reply};

pub fn routes(
    state: &StateHandle,
    conf: &ConfigHandle,
    db: &Db,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    index()
        .or(resource_editor())
        .or(resources(conf, db))
        .or(file(conf, db))
        .or(items(state))
}

fn index() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path::end()
        .and(warp::get())
        .and_then(|| async { Ok::<_, Rejection>("Market API Server") })
}

fn resource_editor() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("resEditor")
        .and(warp::get())
        .and_then(endpoints::get_resource_editor)
}

fn resources(
    conf: &ConfigHandle,
    db: &Db,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let get_resource = warp::path!(PercentDecoded)
        .and(warp::get())
        .and(with_db(db))
        .and_then(endpoints::get_resource);

    let get_all_resources = warp::path::end()
        .and(warp::get())
        .and(with_auth(conf))
        .and(with_db(db))
        .and_then(endpoints::get_all_resources);

    let post_resource = warp::path::end()
        .and(warp::post())
        .and(with_auth(conf))
        .and(warp::body::json())
        .and(with_db(db))
        .and_then(endpoints::post_resource);

    let delete_resource = warp::path!(PercentDecoded)
        .and(warp::delete())
        .and(with_auth(conf))
        .and(with_db(db))
        .and_then(endpoints::delete_resource);

    warp::path!("res" / ..).and(
        get_resource
            .or(get_all_resources)
            .or(post_resource)
            .or(delete_resource),
    )
}

fn file(
    conf: &ConfigHandle,
    db: &Db,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let get_file = warp::path!(PercentDecoded)
        .and(warp::get())
        .and(with_db(db))
        .and_then(endpoints::get_file);

    let get_all_files = warp::path::end()
        .and(warp::get())
        .and(with_auth(conf))
        .and(with_db(db))
        .and_then(endpoints::get_all_files);

    let upload_file = warp::path!(PercentDecoded)
        .and(warp::put())
        .and(with_auth(conf))
        .and(warp::body::bytes())
        .and(with_db(db))
        .and_then(endpoints::upload_file);

    let delete_file = warp::path!(PercentDecoded)
        .and(warp::delete())
        .and(with_auth(conf))
        .and(with_db(db))
        .and_then(endpoints::delete_file);

    warp::path!("file" / ..).and(get_file.or(get_all_files).or(upload_file).or(delete_file))
}

fn items(state: &StateHandle) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("all")
        .and(warp::get())
        .and(warp::query())
        .and(with_state(state))
        .and_then(endpoints::get_all_items)
}

fn with_state(
    state: &StateHandle,
) -> impl Filter<Extract = (StateHandle,), Error = std::convert::Infallible> + Clone {
    let state = state.clone();
    warp::any().map(move || state.clone())
}

fn with_db(db: &Db) -> impl Filter<Extract = (Db,), Error = std::convert::Infallible> + Clone {
    let db = db.clone();
    warp::any().map(move || db.clone())
}

fn with_auth(conf: &ConfigHandle) -> impl Filter<Extract = (), Error = Rejection> + Clone {
    middlewares::authenticate(conf.auth_key.clone())
}
