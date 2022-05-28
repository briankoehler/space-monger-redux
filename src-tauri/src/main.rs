#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
#[path = "tree/tree.rs"] mod tree;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![build_tree_command])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn build_tree_command(path: &str) -> Result<tree::Tree, &str> {
    let tree = tree::Tree::build_tree(path);
    if tree.is_err() {
        return Err("An error occurred while building the tree.");
    }
    Ok(tree.unwrap())
}
