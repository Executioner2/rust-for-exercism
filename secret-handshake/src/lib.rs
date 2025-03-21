pub fn actions(n: u8) -> Vec<&'static str> {
    let mut actions = Vec::new();
    for i in 0..5 {
        match n & (1 << i) { 
            1 =>  actions.push("wink"),
            2 =>  actions.push("double blink"),
            4 =>  actions.push("close your eyes"),
            8 =>  actions.push("jump"),
            16 => actions.reverse(),
            _ =>  ()
        }   
    }
    actions
}

