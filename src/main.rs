use std::usize;

fn min(dl: &[i8], now: i8) -> (i8, usize) {
    let mut min = i8::MAX;

    let mut best_index = usize::MAX;
    for (i, d) in dl.iter().enumerate() {
        let rd = (*d).wrapping_sub(now);
        if rd < min {
            min = rd;
            best_index = i;
        }
    }

    println!("now {} min {} index {}", now, min, best_index);

    (min, best_index)
}

fn main() {
    let t = [10, 100, 127];
    let m = min(&t, 0);

    min(&t, 0);
    min(&t, 10);
    min(&t, 11);
    min(&t, 100);
    min(&t, 101);
}

fn min_copy(dl: &[u8], now: u8) -> (u8, usize) {
    let mut min = u8::MAX as u16 + 1;

    let mut best_index = usize::MAX;
    for (i, d) in dl.iter().enumerate() {
        let rd = (*d as u16).wrapping_sub(now as u16);
        if rd < min {
            min = rd;
            best_index = i;
        }
    }

    (min as u8, best_index)
}
