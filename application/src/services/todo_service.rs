
use domain::entities::todo::ToDo;
use infrastructure::repositories::todo_repository::ToDoRepository;

pub struct ToDoService;

impl ToDoService {
    pub fn get_all_todos() -> Option<Vec<ToDo>> {
        ToDoRepository::all()
    }

    pub fn create_todo(content: String) ->  Result<ToDo, &'static str>{
        ToDoRepository::create(content)
    }

    pub fn get_todo(id: i32) -> Result<ToDo, &'static str>{
        ToDoRepository::find_by_id(id)
    }
    pub fn update_todo(id: i32, content: String, is_completed: bool) ->  Result<ToDo, &'static str>{
        ToDoRepository::update(id, content, is_completed)
    }

    pub fn delete_todo(id: i32) -> bool {
        ToDoRepository::delete(id)
    }
}