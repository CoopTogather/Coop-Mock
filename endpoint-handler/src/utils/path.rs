/// Extracts the first scope from a given path.
///
/// This function takes a string `path` and splits it by the '/' delimiter.
/// It returns an `Option<&str>` representing the first scope of the path,
/// or `None` if the path is empty.
///
/// # Arguments
///
/// * `path` - A string slice representing the path.
///
/// # Returns
///
/// An `Option<&str>` representing the first scope of the path, or `None` if the path is empty.
pub fn first_scope(path: &str) -> Option<&str> {
    let mut scopes = path.split('/').collect::<Vec<&str>>();
    scopes.remove(0);

    if scopes.is_empty() {
        return None;
    }

    Some(scopes[0])
}
