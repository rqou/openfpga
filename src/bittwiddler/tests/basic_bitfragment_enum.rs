use bittwiddler::*;

#[bitpattern]
#[bitfragment(dimensions = 1)]
#[pat_bits("0" = 1, "1" = 2)]
#[derive(Debug, PartialEq, Eq)]
enum MyEnum {
    #[bits("00")]
    Choice1,
    #[bits("01")]
    Choice2,
    #[bits("10")]
    Choice3,
    #[bits("11")]
    Choice4,
}

#[test]
fn basic_bitfragment_enum_encode() {
    let mut out = [false; 3];

    let x = MyEnum::Choice2;
    BitFragment::encode(&x, &mut out[..], [0], [false]);
    assert_eq!(out, [false, false, true]);

    let x = MyEnum::Choice3;
    BitFragment::encode(&x, &mut out[..], [0], [false]);
    assert_eq!(out, [false, true, false]);

    // offset
    let mut out = [true; 5];

    let x = MyEnum::Choice2;
    BitFragment::encode(&x, &mut out[..], [1], [false]);
    assert_eq!(out, [true, true, false, true, true]);

    let x = MyEnum::Choice3;
    BitFragment::encode(&x, &mut out[..], [1], [false]);
    assert_eq!(out, [true, true, true, false, true]);

    // mirroring
    let mut out = [false; 3];
    let x = MyEnum::Choice2;
    BitFragment::encode(&x, &mut out[..], [2], [true]);
    assert_eq!(out, [true, false, false]);

    let mut out = [true; 5];
    let x = MyEnum::Choice3;
    BitFragment::encode(&x, &mut out[..], [3], [true]);
    assert_eq!(out, [true, false, true, true, true]);
}

#[test]
fn basic_bitfragment_enum_decode() {
    let x = [true, false, false];
    let out: MyEnum = BitFragment::decode(&x[..], [0], [false]).unwrap();
    assert_eq!(out, MyEnum::Choice1);

    let x = [false, true, true];
    let out: MyEnum = BitFragment::decode(&x[..], [0], [false]).unwrap();
    assert_eq!(out, MyEnum::Choice4);

    // offset
    let x = [false, false, false, true, false, false];
    let out: MyEnum = BitFragment::decode(&x[..], [3], [false]).unwrap();
    assert_eq!(out, MyEnum::Choice1);

    let x = [true, true, true, false, true, true];
    let out: MyEnum = BitFragment::decode(&x[..], [3], [false]).unwrap();
    assert_eq!(out, MyEnum::Choice4);

    // mirroring
    let x = [true, false, false];
    let out: MyEnum = BitFragment::decode(&x[..], [2], [true]).unwrap();
    assert_eq!(out, MyEnum::Choice2);

    let x = [true, true, true, false, true, true];
    let out: MyEnum = BitFragment::decode(&x[..], [5], [true]).unwrap();
    assert_eq!(out, MyEnum::Choice3);
}
