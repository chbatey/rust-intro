pub fn add_one(x: int) -> int {
   x + 1
}

fn add_two(x: int) -> int {
   add_one(add_one(x))
}

#[cfg(test)]
mod test {
    use super::add_two;

    #[test]
    fn test_add_two() {
        let result = add_two(5i);

        assert_eq!(7i, result);
    }
}
