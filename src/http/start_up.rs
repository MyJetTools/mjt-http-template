use std::{net::SocketAddr, sync::Arc};

use is_alive_middleware::IsAliveMiddleware;
use my_http_server::MyHttpServer;
use my_http_server_controllers::swagger::SwaggerMiddleware;

use crate::{AppContext, build_controllers, APP_NAME, APP_VERSION};

pub fn setup_server(app: Arc<AppContext>, port: u16) {
    let mut http_server = MyHttpServer::new(SocketAddr::from(([0, 0, 0, 0], port)));

    let controllers = Arc::new(build_controllers(&app));

    let swagger_middleware = SwaggerMiddleware::new(
        controllers.clone(),
        APP_NAME.to_string(),
        APP_VERSION.to_string(),
    );

    http_server.add_middleware(Arc::new(IsAliveMiddleware::new(
        APP_NAME.to_string(),
        APP_VERSION.to_string(),
    )));

    http_server.add_middleware(Arc::new(swagger_middleware));
    http_server.add_middleware(controllers);

    http_server.start(app.app_states.clone(), my_logger::LOGGER.clone());
}
