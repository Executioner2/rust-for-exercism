use macros::hashmap;
use std::collections::HashMap;

fn main() {
    // three arguments are invalid
    // hashmap!('a' => 1, 'b');
    hashmap!('a' => 1, 'b' => 2);  // 因为报错，我先改对
}
