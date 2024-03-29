use nth_prime as np;

#[test]
fn test_first_prime() {
    assert_eq!(np::nth(0), 2);
}

#[test]
fn test_second_prime() {
    assert_eq!(np::nth(1), 3);
}

#[test]
fn test_sixth_prime() {
    assert_eq!(np::nth(5), 13);
}

#[test]
fn test_twentieth_prime() {
    assert_eq!(np::nth(19), 71);
}

#[test]
fn test_big_prime() {
    assert_eq!(np::nth(10000), 104743);
}

#[test]
fn test_really_big_prime() {
    assert_eq!(np::nth(100_000_000), 2_038_074_751);
}
