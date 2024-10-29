/// returns a list of service repositories
/// TODO: JSON somewhere else
pub fn repo_list() -> Vec<String> {
    let repos = ["mnpqraven/vps-center-hub"];
    let repos_as_str = repos.iter().map(|e| e.to_string()).collect();
    repos_as_str
}
