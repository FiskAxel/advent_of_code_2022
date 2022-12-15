use std::cmp::Ordering;
fn main() {
    let input: String = std::fs::read_to_string("src/input.txt").unwrap();
    let input: String = input.replace("10", ":"); // : comes after 9 in Unicode (characters 0 through : is numbered 48-58)
    let pairs: Vec<&str> = input.split("\r\n\r\n").collect();

    let mut distress_signal: Vec<&[u8]> = vec![];
    let mut sum = 0;
    for (i, pair) in pairs.iter().enumerate() {
        let pair: Vec<&str> = pair.lines().collect();
        let left: &[u8] = pair[0].as_bytes();
        let right: &[u8] = pair[1].as_bytes();
        if compare(left, right) == Ordering::Less {
            sum += i+1; 
        }
        distress_signal.push(left);
        distress_signal.push(right);
    }
    println!("Part 1: {}", sum);

    let divider_packet1: &[u8] = "[[2]]".as_bytes();
    let divider_packet2: &[u8] = "[[6]]".as_bytes();
    distress_signal.push(divider_packet1);
    distress_signal.push(divider_packet2);
    distress_signal.sort_by(|a, b| compare(a, b));
    let index1 = distress_signal.iter().position(|&x| x == divider_packet1).unwrap() + 1;
    let index2 = distress_signal.iter().position(|&x| x == divider_packet2).unwrap() + 1;
    let decoder_key = index1 * index2;
    println!("Part 2: {}", decoder_key);
}

fn compare(left: &[u8], right: &[u8]) -> Ordering {
    match (left[0], right[0]) {
        (l, r) if l == r => compare(&left[1..], &right[1..]),
        
        (b']', _) => Ordering::Less,
        (_, b']') => Ordering::Greater,
        
        (b'[', _) => {
            let r = [&[right[0], b']'], &right[1..]].concat();
            compare(&left[1..], &r[..])
        },
        (_, b'[') => {
            let l: Vec<u8> = [&[left[0], b']'], &left[1..]].concat();
            compare(&l[..], &right[1..])
        },

        (l, r) => if l < r { Ordering::Less }  
                          else { Ordering::Greater }
    }
}
