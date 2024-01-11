use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Role { AnonymousUser, User, Admin, SuperAdmin }
