enum foo { bar }

#[cfg(test)]
mod tests {
    #[test]
    fn print_and_panic() {
        println!("test printed to stdout");
        dbg!("test dbg");
        assert_eq!(5,7);
    }
}