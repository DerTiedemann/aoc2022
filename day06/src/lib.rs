use std::ops::Deref;

// Open ai solution
#[allow(dead_code)]
pub fn find_marker(data: &str, marker_len: usize) -> usize {
    let mut chars = vec![];

    for (i, c) in data.chars().enumerate() {
        chars.push(c);
        if chars.len() > marker_len {
            chars.remove(0);
        }

        if chars.len() == marker_len
            && chars
                .iter()
                .all(|c| chars.iter().filter(|d| d.deref() == c).count() == 1)
        {
            return i + 1;
        }
    }

    data.len()
}
pub fn find_first_n_distict_char_offset(input: &str, n: usize) -> usize {
    let chars = input.trim().chars().collect::<Vec<_>>();
    let offset = chars
        .windows(n)
        .take_while(|w| {
            !w.iter()
                .all(|c| w.iter().filter(|d| d.deref() == c).count() == 1)
        })
        .count();
    n + offset
}
