use crate::structs::ToDoItem;

use glue::errors::NanoServiceError;
#[cfg(feature = "json-file-storage")]
use to_do_dal::json_file::save_one;

pub async fn create(item: ToDoItem) -> Result<ToDoItem, NanoServiceError> {
    save_one(&item.title.to_string(), &item)?;
    Ok(item)
}
