#[cfg(test)]
mod tests {
    use {
        crate::{
            normalise_negative_one_to_one,
        },
    };

    #[test]
    fn test_normalise_negative_one_to_one() {
        assert_eq!(
            normalise_negative_one_to_one(23266.494456592045),
            0.17751689496301304,
        );
        assert_eq!(
            normalise_negative_one_to_one(-23266.494456592045),
            -0.17750163617395054,
        );
    }
}