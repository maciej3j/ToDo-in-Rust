use glue::errors::{NanoServiceError, NanoServiceErrorStatus};
#[cfg(feature = "json-file-storage")]
use to_do_dal::json_file::{get_all, save_all};

use crate::structs::ToDoItem;

pub async fn update(item: ToDoItem) -> Result<(), NanoServiceError> {
    let mut all_items = get_all::<ToDoItem>()?;
    if !all_items.contains_key(&item.title) {
        return Err(NanoServiceError::new(
            format!("Item with name {} not found", item.title),
            NanoServiceErrorStatus::NotFound,
        ));
    }
    all_items.insert(item.title.clone(), item);
    save_all(&all_items)?;
    Ok(())
}
