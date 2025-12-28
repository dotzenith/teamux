mod tmux;

fn main() {

    let manager = tmux::TmuxManager::new().unwrap();

    let sessions = manager.ls().unwrap();
    println!("{:?}", sessions);

}
