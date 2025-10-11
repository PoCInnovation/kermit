mod common;

#[test]
fn test_address_zero() {
    perform_cmd_test!("address_zero", &["utils", "address-zero"]);
}

#[test]
fn test_hash_zero() {
    perform_cmd_test!("hash_zero", &["utils", "hash-zero"]);
}

mod convert {
    use crate::perform_cmd_test;

    #[test]
    fn test_from_atto() {
        perform_cmd_test!("from_atto", &["utils", "convert", "1", "atto"]);
    }

    #[test]
    fn test_from_gatto() {
        perform_cmd_test!("from_gatto", &["utils", "convert", "1", "gatto"]);
    }

    #[test]
    fn test_from_alph() {
        perform_cmd_test!("from_alph", &["utils", "convert", "1", "alph"]);
    }

    #[test]
    fn test_missing_param() {
        perform_cmd_test!("missing_param", &["utils", "convert", "100"]);
    }

    #[test]
    fn test_no_params() {
        perform_cmd_test!("no_params", &["utils", "convert"]);
    }

    #[test]
    fn test_bad_param() {
        perform_cmd_test!("bad_param", &["utils", "convert", "100", "foo"]);
    }
}
