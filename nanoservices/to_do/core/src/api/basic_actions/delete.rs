use glue::errors::NanoServiceError;
use to_do_dal::json_file::delete_one;

use crate::structs::ToDoItem;

pub async fn delete(id: &str) -> Result<ToDoItem, NanoServiceError> {
    delete_one::<ToDoItem>(id)
}
