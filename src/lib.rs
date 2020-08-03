use rand::Rng;

pub fn make_robot_move(current_game: &str) -> String {
    rand::thread_rng().gen_range(0, 3).to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
