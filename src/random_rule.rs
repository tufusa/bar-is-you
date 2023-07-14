extern crate rand;
use super::rule;
use rand::seq::SliceRandom;

impl rule::IsYou {
    fn random() -> Self {
        **Self::RULES.choose(&mut rand::thread_rng()).unwrap()
    }
}

impl rule::IsDeath {
    fn random() -> Self {
        **Self::RULES.choose(&mut rand::thread_rng()).unwrap()
    }
}

impl rule::IsWin {
    fn random() -> Self {
        **Self::RULES.choose(&mut rand::thread_rng()).unwrap()
    }
}

impl rule::IsMove {
    fn random() -> Self {
        **Self::RULES.choose(&mut rand::thread_rng()).unwrap()
    }
}

impl rule::Rule {
    pub(crate) fn random() -> Self {
        Self {
            is_you: rule::IsYou::random(),
            is_death: rule::IsDeath::random(),
            is_win: rule::IsWin::random(),
            is_move: rule::IsMove::random(),
        }
    }
}
