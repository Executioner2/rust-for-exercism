use macros::hashmap;
use std::collections::HashMap;

fn main() {
    // a single trailing comma is okay, but two is not
    // let _hm: HashMap<_, _> = hashmap!('a' => 2, ,);
    let _hm: HashMap<_, _> = hashmap!('a' => 2);  // 因为报错，我先改对
}
