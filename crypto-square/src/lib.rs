/// Encrypt {input:?} using a square code
pub fn encrypt(input: &str) -> String {
    let chars: Vec<char> = input
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect();
    
    let x = (chars.len() as f64).sqrt().ceil() as usize;    
    (0..x).map(|i| {
        chars.chunks(x)
            .filter_map(|v| v.get(i).or(Some(&' ')))
            .collect::<String>()
    }).collect::<Vec<String>>().join(" ")
}