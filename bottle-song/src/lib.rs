pub fn recite(mut start_bottles: u32, mut take_down: u32) -> String {
    let f = |n: u32| -> &str {
        match n {
            1 => "one green bottle",
            2 => "two green bottles",
            3 => "three green bottles",
            4 => "four green bottles",
            5 => "five green bottles",
            6 => "six green bottles",
            7 => "seven green bottles",
            8 => "eight green bottles",
            9 => "nine green bottles",
            10 => "ten green bottles",
            _ => "no green bottles"
        }
    };

    let first_uppers = |s: &str| -> String {
        let mut cs = s.chars();
        if let Some(c) = cs.next() {
            return c.to_ascii_uppercase().to_string() + cs.as_str()
        }
        String::new()
    };

    let mut res = String::new();
    while take_down > 0 {
        let t = f(start_bottles - 1);
        let s = first_uppers(f(start_bottles));

        res.push_str(&format!("{s} hanging on the wall,\n\
        {s} hanging on the wall,\n\
        And if one green bottle should accidentally fall,\n\
        There'll be {t} hanging on the wall."));

        start_bottles -= 1;
        take_down -= 1;
        if take_down > 0 {
            res.push_str("\n\n");
        }
    }
    
    res
}
