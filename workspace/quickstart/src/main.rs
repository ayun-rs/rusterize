use axum::{routing::get, Router};
use ayun::{
    http::{response, Server},
    prelude::*,
};

struct App;

impl ApplicationTrait for App {
    type Service = Server;

    fn with_routing() -> impl ServiceTrait {
        Server::new().register(
            Router::new().merge(
                ayun::http::Router::default()
                    .route("/", get(|| async { response().text("Hello World!") })),
            ),
        )
    }
}

fn main() -> Result<()> {
    bootstrap::<App>()?.run()
}
