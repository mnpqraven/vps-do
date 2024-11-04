/// returns a list of service repositories
/// TODO: JSON somewhere else
pub fn repo_list() -> Vec<String> {
    let repos = ["vps-center-hub", "vps-center-api"];
    let repos_as_str = repos.iter().map(|e| e.to_string()).collect();
    repos_as_str
}
