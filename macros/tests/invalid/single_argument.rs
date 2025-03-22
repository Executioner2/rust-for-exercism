use macros::hashmap;
use std::collections::HashMap;

fn main() {
    // a single argument is invalid
    // let _hm: HashMap<_, _> = hashmap!('a');
    let _hm: HashMap<_, _> = hashmap!('a' => 1); // 因为报错，我先改对
}
