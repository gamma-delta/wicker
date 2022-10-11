use wicker::*;

#[test]
fn double_ended() {
    let picker = WeightedPicker::new(vec![
        ("hello", 0.3),
        ("wicker", 10.0),
        ("this", 3.0),
        ("world", 1.0),
        ("is", 5.0),
    ]);
    eprintln!("{:?}", &picker);
    let mut iter = picker.iter().copied();
    assert_eq!(iter.next_back(), Some("wicker"));
    assert_eq!(iter.next_back(), Some("is"));
    assert_eq!(iter.next_back(), Some("this"));
    assert_eq!(iter.next_back(), Some("world"));
    assert_eq!(iter.next_back(), Some("hello"));
    assert_eq!(iter.next_back(), None);
}

#[test]
fn both_ends() {
    let picker = WeightedPicker::new(vec![
        ("hello", 0.3),
        ("wicker", 10.0),
        ("this", 3.0),
        ("world", 1.0),
        ("is", 5.0),
    ]);
    let mut iter = picker.iter().copied();
    assert_eq!(iter.next_back(), Some("wicker"));
    assert_eq!(iter.next(), Some("hello"));
    assert_eq!(iter.next(), Some("world"));
    assert_eq!(iter.next_back(), Some("is"));
    assert_eq!(iter.next_back(), Some("this"));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next_back(), None);
}

#[test]
fn exact_size() {
    let picker = WeightedPicker::new(vec![
        ("hello", 0.3),
        ("wicker", 10.0),
        ("this", 3.0),
        ("world", 1.0),
        ("is", 5.0),
    ]);
    let mut iter = picker.iter().copied();
    for idx in 0..=picker.len() {
        let expect = picker.len() - idx;
        assert_eq!(expect, iter.len());
        // get a mix of front and back
        if idx % 2 == 0 {
            iter.next();
        } else {
            iter.next_back();
        }
    }
}
