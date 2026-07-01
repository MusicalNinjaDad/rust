#![cfg_attr(unstable_assert_matches, feature(assert_matches))]

#[cfg(test)]
#[cfg(has_assert_matches)]
mod tests {
    #[cfg(assert_matches_location = "root")]
    use std::assert_matches;

    #[cfg(assert_matches_location = "module")]
    use std::assert_matches::assert_matches;

    #[test]
    fn has() {
        assert_matches!(Some(5), Some(n) if n == 5);
    }
}

#[cfg(test)]
#[cfg(not(has_assert_matches))]
mod tests {
    #[test]
    fn has_not() {
        assert_eq!(Some(5), Some(5));
    }
}
