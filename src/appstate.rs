use etag::EntityTag;
use std::sync::RwLock;

#[derive(Debug)]
pub struct AppState {
    pub is_modified: RwLock<bool>,
    pub etag: RwLock<EntityTag>,
}
