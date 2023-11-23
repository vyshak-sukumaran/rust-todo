use actix_web::{delete, get, post, put, web, Error, HttpResponse, Responder, Scope};
use application::{
    domain::domain::entities::todo::{ToDo, ToDoContent},
    services::todo_service::ToDoService,
};

pub fn config(cfg: &mut web::ServiceConfig) {
    let scope: Scope = web::scope("/todos")
        .service(get_todos)
        .service(get_todo)
        .service(create_todo)
        .service(update_todo)
        .service(delete_todo);
    cfg.service(scope);
}

#[get("")]
pub async fn get_todos() -> impl Responder {
    let all_todos: Option<Vec<ToDo>> = ToDoService::get_all_todos();
    match all_todos {
        Some(todos) => HttpResponse::Ok().json(todos),
        None => HttpResponse::NoContent().finish(),
    }
}

#[get("/{id}")]
pub async fn get_todo(path: web::Path<i32>) -> impl Responder {
    let todo: Result<ToDo, &str> = ToDoService::get_todo(path.into_inner());
    match todo {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(_) => HttpResponse::NoContent().finish(),
    }
}

#[post("")]
pub async fn create_todo(todo: web::Json<ToDoContent>) -> impl Responder {
    let result: Result<ToDo, &str> = ToDoService::create_todo(todo.content.clone());
    match result {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(_) => HttpResponse::BadRequest().json("Bad Request"),
    }
}

#[put("/{id}")]
pub async fn update_todo(path: web::Path<i32>, todo: web::Json<ToDoContent>) -> impl Responder {
    let result: Result<ToDo, &str> =
        ToDoService::update_todo(path.into_inner(), todo.content.clone(), todo.is_completed);
    match result {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(_) => HttpResponse::BadRequest().json("Bad Request"),
    }
}

#[delete("/{id}")]
pub async fn delete_todo(path: web::Path<i32>) -> Result<HttpResponse, Error> {
    let result: bool = ToDoService::delete_todo(path.into_inner());
    match result {
        true => Ok(HttpResponse::Ok().json("Todo deleted successfully")),
        false => Ok(HttpResponse::BadRequest().json("Failed to delete todo")),
    }
}