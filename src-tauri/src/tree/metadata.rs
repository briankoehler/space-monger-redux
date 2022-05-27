use std::fs;
use std::time;

/// Data related to a node in a tree.
/// 
/// Although similar to [`std::fs::Metadata`](https://doc.rust-lang.org/stable/std/fs/struct.Metadata.html), 
/// this makes it easier to test.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Metadata {
    pub name: String,
    pub size: u64,
    pub permissions: fs::Permissions,
    pub modified: Option<time::SystemTime>,
    pub accessed: Option<time::SystemTime>,
    pub created: Option<time::SystemTime>
}
