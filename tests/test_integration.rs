
use xensieve::Sieve;
use xensieve_sandbox::sieve_to_str;


//------------------------------------------------------------------------------
#[test]
fn test_sieve_new_a() {
    let s = sieve_to_str("3@0|5@1|5@4".to_string());
    assert_eq!(s, "Sieve{3@0|5@1|5@4}");
}


#[test]
fn test_sieve_iter_int_h() {
    let s1 = Sieve::new("7@0 | (!5@2 & !4@3)");
    let post1: Vec<_> = s1.iter_value(0..=12).collect();
    assert_eq!(post1, vec![0, 1, 4, 5, 6, 7, 8, 9, 10]);
}

