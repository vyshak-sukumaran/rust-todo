use actix_web::web;
mod todo_controller;

pub fn config(cfg: &mut web::ServiceConfig) {
    let scope = web::scope("/api").configure(todo_controller::config);
    cfg.service(scope);
}
