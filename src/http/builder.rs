use std::sync::Arc;

use my_http_server_controllers::controllers::ControllersMiddleware;
use crate::{AppContext, TestControllerAction};

pub fn build_controllers(app: &Arc<AppContext>) -> ControllersMiddleware {
    let mut result = ControllersMiddleware::new();

    result.register_get_action(Arc::new(
        TestControllerAction::new(
            app.clone(),
        ),
    ));

    result
}
