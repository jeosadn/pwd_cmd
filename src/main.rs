use std::env;

fn main() {
    let mut current_dir: String = env::current_dir()
        .expect("Invalid dir")
        .to_str()
        .unwrap()
        .to_string();

    let max_len = 20;
    let dir_len = current_dir.chars().count();

    if dir_len > max_len+1 {
        current_dir = "…".to_string() + &current_dir[dir_len-max_len+1..dir_len];
    }

    println!("{}", current_dir);
}
