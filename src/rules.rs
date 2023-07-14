use super::rule;

impl rule::IsYou {
    pub(crate) const RULES: [&'static Self; 2] = [&Self::Bar, &Self::Ball];
}

impl rule::IsDeath {
    pub(crate) const RULES: [&'static Self; 2] = [&Self::Out, &Self::Block];
}

impl rule::IsWin {
    pub(crate) const RULES: [&'static Self; 2] = [&Self::None, &Self::BreakAll];
}

impl rule::IsMove {
    pub(crate) const RULES: [&'static Self; 3] = [&Self::Bar, &Self::Ball, &Self::Block];
}
