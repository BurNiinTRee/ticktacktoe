#[cfg(test)]
#[test]
fn Ttt_is_won() {
    use super::ttt::Field::{Circle, Cross, Empty};
    use super::ttt::Ttt;
    let mut game = Ttt::new();
    game.board = [Circle, Circle, Circle, Empty, Empty, Empty, Empty, Empty, Empty];
    assert_eq!(game.is_won(), Circle);
}
