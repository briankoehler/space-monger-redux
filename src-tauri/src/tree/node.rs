#[path = "./metadata.rs"] mod metadata;
use std::fs;

/// The children variants of a tree.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
pub enum Node {
    File(File),
    Directory(Directory)
}

impl Node {
    /// Creates a new [`File`] Node.
    /// 
    /// # Arguments
    /// 
    /// * `name` - A string slice that holds the name of the file.
    /// 
    /// * `metadata` - Data about the file.
    /// 
    /// # Example
    /// 
    /// ```
    /// let metadata = std::fs::metadata("./foo").unwrap();
    /// let foo = Node::new_file("foo", metadata);
    /// ```
    pub fn new_file(name: &str, metadata: &fs::Metadata) -> Self {
        Node::File(File::new(name, metadata))
    }
    
    /// Creates a new [`Directory`] Node.
    /// 
    /// # Arguments
    /// 
    /// * `name` - A string slice that holds the name of the file.
    /// 
    /// * `metadata` - Data about the file.
    /// 
    /// # Example
    /// 
    /// ```
    /// let metadata = std::fs::metadata("./bar").unwrap();
    /// let bar = Node::new_directory("bar", metadata);
    /// ```
    pub fn new_directory(name: &str, metadata: &fs::Metadata) -> Self {
        Node::Directory(Directory::new(name, metadata))
    }

    /// Fetch a string slice of the name of this node. Useful if you have a [`Node`] type 
    /// instead of a [`File`] or [`Directory`].
    pub fn get_name(&self) -> &str {
        match self {
            Node::Directory(d) => &d.metadata.name,
            Node::File(f) => &f.metadata.name
        }
    }

    /// Fetch the metadata of this node. Useful if you have a [`Node`] type 
    /// instead of a [`File`] or [`Directory`].
    pub fn get_metadata(&self) -> &metadata::Metadata {
        match self {
            Node::Directory(d) => &d.metadata,
            Node::File(f) => &f.metadata
        }
    }

    /// Get a reference to the node as a [`File`].
    /// 
    /// # Example
    /// 
    /// ```
    /// // Assume we have a Node called foo.
    /// let foo_file = foo.as_file().unwrap();
    /// ```
    pub fn as_file(&mut self) -> Option<&mut File> {
        match self {
            Node::File(f) => Some(f),
            _ => None
        }
    }

    /// Get a reference to the node as a [`Directory`].
    /// 
    /// # Example
    /// 
    /// ```
    /// // Assume we have a Node called bar.
    /// let bar_dir = bar.as_directory().unwrap();
    /// bar_dir.add_child(foo);
    /// ```
    pub fn as_directory(&mut self) -> Option<&mut Directory> {
        match self {
            Node::Directory(d) => Some(d),
            _ => None
        }
    }

    fn handle_metadata_result<T>(data: std::result::Result<T, std::io::Error>) -> Option<T> {
        if let Ok(i) = data {
            return Some(i);
        }
        None
    }
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
pub struct File {
    pub metadata: metadata::Metadata
}

impl File {
    fn new(name: &str, metadata: &fs::Metadata) -> Self {
        File {
            metadata: metadata::Metadata {
                name: name.to_owned(),
                size: metadata.len(),
                // permissions: metadata.permissions(),
                modified: Node::handle_metadata_result(metadata.modified()),
                accessed: Node::handle_metadata_result(metadata.accessed()),
                created: Node::handle_metadata_result(metadata.created())
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
pub struct Directory {
    pub metadata: metadata::Metadata,
    pub children: Vec<Node>
}

impl Directory {
    fn new(name: &str, metadata: &fs::Metadata) -> Self {
        Directory {
            metadata: metadata::Metadata {
                name: name.to_owned(),
                size: metadata.len(),
                // permissions: metadata.permissions(),
                modified: Node::handle_metadata_result(metadata.modified()),
                accessed: Node::handle_metadata_result(metadata.accessed()),
                created: Node::handle_metadata_result(metadata.created())
            },
            children: Vec::new()
        }
    }

    pub fn add_child(&mut self, child: Node) { self.children.push(child); }
}
