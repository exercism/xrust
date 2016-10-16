#[test]
fn roll_returns_a_result() {
    let game = BowlingGame::new();
    assert!(game.roll(0).is_ok());
}

#[test]
fn you_can_not_roll_more_than_ten_pins_in_a_single_roll() {
    let game = BowlingGame::new();

    assert!(game.roll(11).is_err());
}

#[test]
fn a_game_score_is_ok_if_ten_frames_have_been_rolled() {
    let game = BowlingGame::new();

    for x in (0..10) {
        game.roll(0).is_ok();
        game.roll(0).is_ok();
    }

    assert!(game.score().is_ok());
}

#[test]
fn you_can_not_score_a_game_with_no_rolls() {
    let game = BowlingGame::new();

    assert!(game.score().is_err());
}

#[test]
fn a_game_score_is_err_if_fewer_than_ten_frames_have_been_rolled() {
    let game = BowlingGame::new();

    for x in (0..9) {
        game.roll(0).is_ok();
        game.roll(0).is_ok();
    }

    assert!(game.score().is_err());
}

#[test]
fn a_game_score_is_err_if_more_than_ten_frames_have_been_rolled() {
    let game = BowlingGame::new();

    for x in (0..11) {
        game.roll(0).is_ok();
        game.roll(0).is_ok();
    }

    assert!(game.score().is_err());
}

#[test]
fn twenty_zero_pin_rolls_scores_zero() {
    let game = BowlingGame::new();

    for x in (0..20) {
        game.roll(0).is_ok();
    }

    assert_eq!(game.score().unwrap(), 0);
}

#[test]
fn ten_frames_without_a_strike_or_spare() {
    let game = BowlingGame::new();

    for x in (0..10) {
        game.roll(3).is_ok();
        game.roll(6).is_ok();
    }

    assert_eq!(game.score().unwrap(), 90);
}

#[test]
fn spare_in_the_first_frame_followed_by_zeros() {
    let game = BowlingGame::new();

    game.roll(6).is_ok();
    game.roll(4).is_ok();

    for x in (0..18) {
        game.roll(0).is_ok();
    }

    assert_eq!(game.score().unwrap(), 10);
}

#[test]
fn points_scored_in_the_roll_after_a_spare_are_counted_twice_as_a_bonus() {
    let game = BowlingGame::new();

    game.roll(6).is_ok();
    game.roll(4).is_ok();
    game.roll(3).is_ok();

    for x in (0..17) {
        game.roll(0).is_ok();
    }

    assert_eq!(game.score().unwrap(), 16);
}

#[test]
fn consecutive_spares_each_get_a_one_roll_bonus() {
    let game = BowlingGame::new();

    game.roll(5).is_ok();
    game.roll(5).is_ok();
    game.roll(3).is_ok();
    game.roll(7).is_ok();
    game.roll(4).is_ok();

    for x in (0..15) {
        game.roll(0).is_ok();
    }

    assert_eq!(game.score().unwrap(), 31);
}

#[test]
fn if_the_last_frame_is_a_spare_you_get_one_extra_roll_that_is_scored_once() {
    let game = BowlingGame::new();

    for x in (0..18) {
        game.roll(0).is_ok();
    }

    game.roll(5).is_ok();
    game.roll(5).is_ok();
    game.roll(7).is_ok();

    assert_eq!(game.score().unwrap(), 17);
}

#[test]
fn a_strike_earns_ten_points_in_a_frame_with_a_single_roll() {
    let game = BowlingGame::new();

    game.roll(10).is_ok();

    for x in (0..18) {
        game.roll(0).is_ok();
    }

    assert_eq!(game.score().unwrap(), 10);
}

#[test]
fn points_scored_in_the_two_rolls_after_a_strike_are_counted_twice_as_a_bonus() {
    let game = BowlingGame::new();

    game.roll(10).is_ok();
    game.roll(5).is_ok();
    game.roll(3).is_ok();

    for x in (0..16) {
        game.roll(0).is_ok();
    }

    assert_eq!(game.score().unwrap(), 26);
}

#[test]
fn consecutive_strikes_each_get_the_two_roll_bonus() {
    let game = BowlingGame::new();

    game.roll(10).is_ok();
    game.roll(10).is_ok();
    game.roll(10).is_ok();
    game.roll(5).is_ok();
    game.roll(3).is_ok();

    for x in (0..12) {
        game.roll(0).is_ok();
    }

    assert_eq!(game.score().unwrap(), 81);
}

#[test]
fn a_strike_in_the_last_frame_earns_a_two_roll_bonus_that_is_counted_once() {
    let game = BowlingGame::new();

    for x in (0..18) {
        game.roll(0).is_ok();
    }

    game.roll(10).is_ok();
    game.roll(7).is_ok();
    game.roll(1).is_ok();

    assert_eq!(game.score().unwrap(), 18);
}

#[test]
fn a_spare_with_the_two_roll_bonus_does_not_get_a_bonus_roll() {
    let game = BowlingGame::new();

    for x in (0..18) {
        game.roll(0).is_ok();
    }

    game.roll(10).is_ok();
    game.roll(7).is_ok();
    game.roll(3).is_ok();

    assert_eq!(game.score().unwrap(), 20);
}

#[test]
fn strikes_with_the_two_roll_bonus_do_not_get_a_bonus_roll() {
    let game = BowlingGame::new();

    for x in (0..18) {
        game.roll(0).is_ok();
    }

    game.roll(10).is_ok();
    game.roll(10).is_ok();
    game.roll(10).is_ok();

    assert_eq!(game.score().unwrap(), 30);
}

#[test]
fn a_strike_with_the_one_roll_bonus_after_a_spare_in_the_last_fram_does_not_get_a_bonus() {
    let game = BowlingGame::new();

    for x in (0..18) {
        game.roll(0).is_ok();
    }

    game.roll(7).is_ok();
    game.roll(3).is_ok();
    game.roll(10).is_ok();

    assert_eq!(game.score().unwrap(), 20);
}

#[test]
fn all_strikes_is_a_perfect_score_of_300() {
    let game = BowlingGame::new();

    for x in (0..12) {
        game.roll(10).is_ok();
    }

    assert_eq!(game.score().unwrap(), 300);
}

#[test]
fn you_can_not_roll_more_than_ten_pins_in_a_single_frame() {
    let game = BowlingGame::new();

    assert!(game.roll(5).is_ok());
    assert!(game.roll(6).is_err());
}

#[test]
fn you_can_not_roll_more_than_ten_pins_with_the_two_bonus_rolls_after_a_final_strike() {
    let game = BowlingGame::new();

    for x in (0..18) {
        game.roll(0).is_ok();
    }

    game.roll(10).is_ok();

    assert!(game.roll(5).is_ok());
    assert!(game.roll(6).is_err());
}

#[test]
fn if_the_last_frame_is_a_strike_you_can_not_score_before_the_extra_rolls_are_taken() {
    let game = BowlingGame::new();

    for x in (0..18) {
        game.roll(0).is_ok();
    }

    game.roll(10).is_ok();

    assert!(game.score().is_err());

    game.roll(10).is_ok();

    assert!(game.score().is_err());

    game.roll(10).is_ok();

    assert!(game.score().is_ok());
}

#[test]
fn if_the_last_frame_is_a_spare_you_can_not_create_a_score_before_extra_roll_is_taken() {
    let game = BowlingGame::new();

    for x in (0..18) {
        game.roll(0).is_ok();
    }

    game.roll(5).is_ok();
    game.roll(5).is_ok();

    assert!(game.score().is_err());

    game.roll(10).is_ok();

    assert!(game.score().is_ok());
}
