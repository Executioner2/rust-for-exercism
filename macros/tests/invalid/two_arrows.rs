use macros::hashmap;

fn main() {
    // a trailing => isn't valid either
    // hashmap!('a' => 2, =>);
    hashmap!('a' => 2); // 因为报错，我先改对
}
