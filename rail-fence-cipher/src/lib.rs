pub struct RailFence(u32);

impl RailFence {
    pub fn new(rails: u32) -> RailFence {
        Self(rails)
    }

    pub fn encode(&self, text: &str) -> String {
        let text: Vec<char> = text.chars().collect();
        let n = self.0 as usize * 2 - 2;
        let mut str = String::with_capacity(text.len());
        for i in 0..self.0 as usize {
            let mut k = n - i * 2;
            let mut j = i;
            while j < text.len() {
                str.push(text[j]);
                if k == 0 {
                    k = n;
                }
                j += k;
                k = n - k;
            }
        }
        str
    }

    pub fn decode(&self, ciphertext: &str) -> String {
        if ciphertext.is_empty() {
            return String::new();
        }

        let ciphertext: Vec<char> = ciphertext.chars().collect();
        let n = self.0 as usize * 2 - 2;
        let k = ciphertext.len() / n;
        let mut m = ciphertext.len() % n;
        let mut ars = Vec::with_capacity(self.0 as usize);
        let mut prev = 0;

        // 确定每个栏的起始位置和截止位置
        for i in 0..self.0 as usize {
            let mut len = k;
            if i > 0 && i < self.0 as usize - 1 { len *= 2; }
            if m > 0 { len += 1; m -= 1; }
            ars.push((prev, prev + len));
            prev += len;
        }

        let mut str = String::with_capacity(ciphertext.len());
        let mut it = (0..self.0 as usize)
            .chain((1..self.0 as usize - 1).rev()).cycle();
        loop {
            let k = it.next().unwrap();
            if ars[k].0 >= ars[k].1 {
                break;
            }
            str.push(ciphertext[ars[k].0]);
            ars[k].0 += 1;
        }

        str
    }
}
