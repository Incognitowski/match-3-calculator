pub mod board;
pub mod constants;
pub mod gem;
pub mod gem_color;
pub mod gem_combination;
pub mod utilities;

#[cfg(test)]
mod tests {
    use crate::utilities::board_from_string;

    #[test]
    fn should_find_no_combinations() {
        let board_str = r#"
            🟦🟨🟦🟩🟨🟥🟪🟨
            🟦🟨🟦🟨🟪🟥🟪🟩
            🟩🟦🟥🟥🟨🟨🟦🟨
            🟪🟨🟦🟪🟩🟨🟨🟪
            🟦🟦🟩🟦🟩🟥🟪🟥
            🟥🟥🟩🟩🟦🟦🟪🟨
            🟪🟩🟪🟩🟪🟦🟨🟪
            🟥🟦🟩🟪🟨🟨🟦🟥
        "#;
        let board = board_from_string(board_str);
        assert!(board.find_combinations().is_empty())
    }

    #[test]
    fn should_find_four_combinations() {
        let board_str = r#"
            🟨🟥🟪🟥🟨🟩🟩🟨
            🟨🟪🟥🟩🟦🟥🟦🟥
            🟥🟥🟩🟪🟦🟨🟩🟪
            🟥🟦🟦🟦🟨🟩🟩🟦
            🟩🟩🟩🟩🟥🟥🟥🟨
            🟩🟥🟩🟦🟩🟩🟨🟨
            🟦🟦🟨🟨🟦🟨🟨🟦
            🟦🟦🟥🟦🟪🟩🟨🟨
        "#;
        let board = board_from_string(board_str);
        assert_eq!(4, board.find_combinations().len())
    }

    #[test]
    fn should_find_zero_combinations_and_no_possible_combinations() {
        let board_str = r#"
            🟨🟦🟨🟦🟨🟦🟨🟦
            🟨🟦🟨🟦🟨🟦🟨🟦
            🟩🟥🟩🟥🟩🟥🟩🟥
            🟩🟥🟩🟥🟩🟥🟩🟥
            🟪🟦🟪🟦🟪🟦🟪🟦
            🟪🟦🟪🟦🟪🟦🟪🟦
            🟨🟩🟨🟩🟨🟩🟨🟩
            🟨🟩🟨🟩🟨🟩🟨🟩
        "#;
        let board = board_from_string(board_str);
        assert!(board.find_combinations().is_empty());
        assert!(!board.contains_possible_combinations());
    }

    #[test]
    fn should_find_zero_combinations_and_possible_combinations_v1() {
        let board_str = r#"
            🟨🟦🟨🟦🟨🟦🟨🟦
            🟨🟦🟨🟦🟨🟦🟨🟦
            🟩🟥🟩🟥🟩🟥🟩🟥
            🟩🟥🟩🟥🟩🟥🟩🟥
            🟪🟦🟪🟩🟪🟦🟪🟦
            🟪🟦🟪🟦🟪🟦🟪🟦
            🟨🟩🟨🟩🟨🟩🟨🟩
            🟨🟩🟨🟩🟨🟩🟨🟩
        "#;
        let board = board_from_string(board_str);
        assert!(board.find_combinations().is_empty());
        assert!(board.contains_possible_combinations());
    }

    #[test]
    fn should_find_zero_combinations_and_possible_combinations_v2() {
        let board_str = r#"
            🟨🟦🟩🟦🟨🟦🟨🟦
            🟨🟦🟨🟦🟨🟦🟨🟦
            🟩🟥🟩🟥🟩🟥🟩🟥
            🟩🟥🟩🟥🟩🟥🟩🟥
            🟪🟦🟪🟦🟪🟦🟪🟦
            🟪🟦🟪🟦🟪🟦🟪🟦
            🟨🟩🟨🟩🟨🟩🟨🟩
            🟨🟩🟨🟩🟨🟩🟨🟩
        "#;
        let board = board_from_string(board_str);
        assert!(board.find_combinations().is_empty());
        assert!(board.contains_possible_combinations());
    }

    #[test]
    fn should_find_zero_combinations_and_possible_combinations_v3() {
        let board_str = r#"
            🟨🟦🟪🟦🟨🟦🟨🟦
            🟨🟦🟪🟦🟨🟦🟨🟦
            🟩🟨🟩🟥🟩🟥🟩🟥
            🟩🟥🟩🟥🟩🟥🟩🟥
            🟪🟦🟪🟦🟪🟦🟪🟦
            🟪🟦🟪🟦🟪🟦🟪🟦
            🟨🟩🟨🟩🟨🟩🟨🟩
            🟨🟩🟨🟩🟨🟩🟨🟩
        "#;
        let board = board_from_string(board_str);
        assert!(board.find_combinations().is_empty());
        assert!(board.contains_possible_combinations());
    }

    #[test]
    fn should_find_zero_combinations_and_possible_combinations_v4() {
        let board_str = r#"
            🟨🟦🟨🟦🟨🟦🟨🟦
            🟨🟦🟨🟦🟨🟦🟨🟦
            🟩🟥🟩🟥🟩🟥🟩🟥
            🟩🟥🟩🟥🟩🟥🟩🟥
            🟪🟦🟪🟦🟪🟦🟪🟦
            🟪🟨🟪🟦🟪🟦🟪🟦
            🟨🟩🟥🟩🟨🟩🟨🟩
            🟨🟩🟥🟩🟨🟩🟨🟩
        "#;
        let board = board_from_string(board_str);
        assert!(board.find_combinations().is_empty());
        assert!(board.contains_possible_combinations());
    }

    #[test]
    fn should_find_zero_combinations_and_possible_combinations_v5() {
        let board_str = r#"
            🟨🟦🟨🟦🟨🟦🟨🟦
            🟨🟦🟨🟦🟨🟦🟨🟦
            🟩🟥🟩🟥🟩🟥🟩🟥
            🟩🟥🟩🟥🟩🟥🟩🟥
            🟪🟦🟪🟦🟪🟦🟪🟦
            🟪🟦🟪🟩🟪🟦🟪🟦
            🟨🟥🟩🟨🟨🟩🟥🟩
            🟨🟥🟥🟩🟨🟩🟨🟩
        "#;
        let board = board_from_string(board_str);
        assert!(board.find_combinations().is_empty());
        assert!(board.contains_possible_combinations());
    }

    #[test]
    fn should_find_zero_combinations_and_possible_combinations_v6() {
        let board_str = r#"
            🟨🟦🟨🟦🟨🟦🟨🟦
            🟨🟦🟨🟦🟨🟦🟨🟦
            🟩🟥🟩🟥🟩🟥🟩🟥
            🟩🟥🟩🟥🟩🟥🟩🟥
            🟪🟦🟪🟨🟦🟨🟪🟦
            🟪🟦🟪🟦🟪🟦🟪🟦
            🟨🟥🟥🟩🟨🟩🟥🟩
            🟨🟥🟥🟩🟨🟩🟨🟩
        "#;
        let board = board_from_string(board_str);
        assert!(board.find_combinations().is_empty());
        assert!(board.contains_possible_combinations());
    }
}
