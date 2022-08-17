use crate::model::User;
use etag::EntityTag;
use std::sync::RwLock;

#[derive(Debug)]
pub struct AppState {
    pub phonebook_entries: RwLock<Vec<User>>,
    pub is_modified: RwLock<bool>,
    pub etag: RwLock<EntityTag>,
}
