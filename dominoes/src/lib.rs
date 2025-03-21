use std::collections::HashSet;

pub fn chain(input: &[(u8, u8)]) -> Option<Vec<(u8, u8)>> {
    if input.is_empty() {
        return Some(vec![]);
    } else if input.len() == 1 {
        return if input[0].0 == input[0].1 {
            Some(vec![input[0]])
        } else {
            None
        }
    }
    
    let mut stack = Vec::new();
    let mut visited: HashSet<usize> = HashSet::new();
    stack.push(input[0]);
    visited.insert(0);
    
    if dfs(&mut stack, &mut visited, input) {
        return Some(stack);
    }
    
    None
}

fn dfs(stack: &mut Vec<(u8, u8)>, visited: &mut HashSet<usize>, input: &[(u8, u8)]) -> bool {
    if visited.len() == input.len() {
        return true;
    }
    let b = stack[stack.len() - 1].1;
    for (i, item) in input.iter().enumerate() {
        if visited.contains(&i) {
            continue;
        }
        if item.0 == b || item.1 == b {
            if item.0 == b {
                stack.push(*item);    
            } else {
                stack.push((item.1, item.0));
            }            
            visited.insert(i);
            if dfs(stack, visited, input) {
                return true;
            }
            stack.pop();
            visited.remove(&i);
        }
    }
    false
}

// ===========================================================================
// 大哥写的
// ===========================================================================

// use std::collections::HashMap;
// 
// /// Linear runtime: Based on a search of an Eulerian tour using a variation of
// /// Hierholzer's algorithm.
// pub fn chain(input: &[(u8, u8)]) -> Option<Vec<(u8, u8)>> {
//     let mut num2stone = HashMap::new();
//     for (i, &(a, b)) in input.iter().enumerate() {
//         num2stone.entry(a).or_insert(Vec::new()).push(i);
//         num2stone.entry(b).or_insert(Vec::new()).push(i);
//     }
//     if num2stone.values().any(|d| d.len() & 1 != 0) {
//         return None; // not a Eulerian graph => no chain exists
//     }
//     let (mut path, mut cycle) = (Vec::new(), Vec::new());
//     let mut num = match num2stone.keys().next() {
//         None => return Some(vec![]),
//         Some(&num) => num,
//     };
//     loop {
//         match num2stone.get_mut(&num).unwrap().pop() {
//             Some(idx) => {
//                 let (a, b) = input[idx];
//                 let (a, b) = if a == num { (a, b) } else { (b, a) };
//                 num = b;
//                 num2stone.get_mut(&num).unwrap().retain(|&x| x != idx);
//                 path.push((a, b));
//             }
//             None => match path.pop() {
//                 None => break,
//                 Some((a, b)) => {
//                     cycle.push((b, a));
//                     num = a;
//                 }
//             },
//         }
//     }
//     if cycle.len() == input.len() {
//         Some(cycle)
//     } else {
//         None
//     }
// }
