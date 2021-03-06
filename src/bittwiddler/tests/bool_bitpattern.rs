use bittwiddler::*;

#[test]
fn bool_bitpattern_encode() {
    let x = true;
    assert_eq!(x.encode(()), [true]);

    let x = false;
    assert_eq!(x.encode(()), [false]);
}

#[test]
fn bool_bitpattern_decode() {
    let x = [true];
    assert_eq!(bool::decode(&x, ()).unwrap(), true);

    let x = [false];
    assert_eq!(bool::decode(&x, ()).unwrap(), false);
}

#[test]
fn bool_bitpattern_docs() {
    let reference =r#"0 |       |
--+-------+------
0 | false | false
1 | true  | true
"#;

    assert_eq!(reference, bool::docs_as_ascii_table());
}
