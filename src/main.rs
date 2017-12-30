fn main() {

    let mut colors = vec![vec![25u8, 12u8, 10u8]];

    prepare_array(&mut colors, 3);
}

fn prepare_array(mut vec: &mut Vec<Vec<u8>>, dimension: u8) {

    fn sort_split(mut v: &mut [Vec<u8>], current_dim: u8, max_dim: u8) {
        if v.len() == 1 {
            ()
        }

        v.sort_unstable_by_key(|k| k[current_dim as usize]);
        let mut left = &v[..v.len()/2];
        let mut right = &v[v.len()/2..];
        //let (left, right)  = v.split_at((v.len() / 2) as usize);
        //println!("left: {:?}\nright: {:?}", left, right);
        let next_dim = (current_dim + 1) % max_dim;
        sort_split(&mut &left, next_dim, max_dim);
        sort_split(&mut &right, next_dim, max_dim);
    }

    println!("this function was called");
    vec.push(vec![1u8, 1u8, 1u8]);
    vec.push(vec![1u8, 1u8, 1u8]);
    vec.push(vec![1u8, 1u8, 1u8]);
    vec.push(vec![1u8, 1u8, 1u8]);
    sort_split(&mut vec[..], 0, dimension - 1);
}
