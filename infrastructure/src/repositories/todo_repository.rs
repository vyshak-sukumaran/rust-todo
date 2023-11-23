use crate::db::todo_context::TODO_LIST;
use domain::entities::todo::ToDo;

pub struct ToDoRepository;

impl ToDoRepository {
    pub fn all() -> Option<Vec<ToDo>> {
        unsafe {
            let query = TODO_LIST.clone();
            Some(query)
        }
    }

    pub fn find_by_id(id: i32) -> Result<ToDo, &'static str> {
        unsafe {
            let query: Option<&ToDo> = TODO_LIST.iter().find(|&td| td.id == id);
            let result: ToDo = match query {
                None => return Err("Todo Not found!"),
                Some(todo) => todo.clone()
            };
            Ok(result)
        }
    }

    pub fn create(content: String) -> Result<ToDo, &'static str> {
        unsafe {
            let last_todo_id = TODO_LIST.iter().last().map(|td| td.id).unwrap_or_default();
            let new_id: i32 = last_todo_id + 1;

            let todo = ToDo {
                id: new_id,
                content,
                is_completed: false,
            };
            let cloned_todo = todo.clone();
            TODO_LIST.push(todo);
            Ok(cloned_todo)
        }
    }
    pub fn update(id: i32, content: String, is_completed: bool) -> Result<ToDo, &'static str> {
        unsafe {
            let index = TODO_LIST.iter().position(|td| td.id == id);
             match index {
                None => return Err("Todo Not found!"),
                Some(idx) => {
                    TODO_LIST[idx].content = content;
                    TODO_LIST[idx].is_completed = is_completed;
                    return Ok(TODO_LIST[idx].clone())
                }
            };
        }
    }
    pub fn delete(id: i32) -> bool {
        unsafe {
            let index = TODO_LIST.iter().position(|td| td.id == id);
            match index {
                None => return  false,
                Some(idx) => {
                    TODO_LIST.remove(idx);
                    return true;
                }
            };
        }
    }
}
