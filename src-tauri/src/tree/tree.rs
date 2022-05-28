mod node;
use node::Node;
use std::fs;

/// A struct for representing files on the OS.
#[derive(Debug, serde::Serialize)]
pub struct Tree {
    pub root: Node,
}

impl Tree {
    /// Build a new `tree` structure.
    /// 
    /// # Arguments
    /// 
    /// * `path` - A string slice denoting the path of the root.
    pub fn build_tree(path: &str) -> std::io::Result<Tree> {
        // Get metadata and children
        let children = fs::read_dir(path)?;
        let metadata = fs::metadata(path)?;

        // Determine root name - does not currently account for '/' characters in file names
        let root_name = path.split('/').collect::<Vec<&str>>();
        let root_name = root_name.last().unwrap();
    
        // Construct the node and dir reference
        let mut root = Node::new_directory(root_name, &metadata);
        let root_dir = root.as_directory().unwrap();

        // Keep track of children sizes
        let mut total_children_size = 0;
        
        // Add children
        for child in children {
            let child = child?;

            // Determine name of child
            let name = child.file_name();
            let name = name.to_str().unwrap();
    
            // If child is a file, build file node and add it
            if child.file_type()?.is_file() {
                let child_node = Node::new_file(name, &child.metadata()?);
                total_children_size += child.metadata()?.len();
                root_dir.add_child(child_node);
            }
            // If child is a dir, perform recursion and add it
            if child.file_type()?.is_dir() {
                let tmp_tree = Self::build_tree(child.path().to_str().unwrap())?;
                total_children_size += tmp_tree.root.get_metadata().size;
                root_dir.add_child(tmp_tree.root);
            }
        }

        // Set directory size to sum of children sizes
        root_dir.metadata.size = total_children_size;
    
        // Provide the new tree :)
        Ok(Tree { root })
    }

    /// Get a [`Node`] reference to the file at the specified path in the tree.
    /// 
    /// # Arguments
    /// 
    /// * `path` - String slice of location of desired node
    /// 
    /// # Example
    /// ```
    /// let foo = tree.get_node_at("src/foo.rs");
    /// ```
    pub fn get_node_at(&self, path: &str) -> Option<&Node> {
        // Determine different node names
        let mut node_names: Vec<&str> = path.split('/').collect();

        let mut curr_node = &self.root;

        // If unable to locate next node, return None
        if node_names[0] != curr_node.get_name() { return None; }

        loop {
            if let Node::Directory(d) = curr_node {
                curr_node = d.children.iter().find(|child| child.get_name() == node_names[1]).unwrap();
                node_names.remove(0);
            }
            if node_names[0] == curr_node.get_name() && node_names.len() == 1 { break; }
        }

        Some(curr_node)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_node_at_works() -> std::io::Result<()> {
        let tree = Tree::build_tree("./resources/test_files/src")?;
        assert_eq!(tree.get_node_at("src/main.py").unwrap().get_name(), "main.py");
        Ok(())
    }
}
