//use core::slice::sort;

//use cami::{COrd, CPartialEq, Locality};
use cami::Cami;

#[test]
pub fn strs() {
    const UNSORTED: [&str; 7] = ["Hello", "mate", "How", "is", "everyone", "at", "home?"];

    let mut sorted_unstable = Vec::with_capacity(7);
    sorted_unstable.extend(UNSORTED.iter().cloned().map(|s| Cami::new(s)));
    sorted_unstable.sort_unstable();
    assert_eq!(sorted_unstable, ["at", "is", "How", "mate", "Hello", "home?", "everyone"].iter().cloned().map(|s| Cami::new(s)).collect::<Vec<_>>());
}
