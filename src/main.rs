fn main() {

    let mut colors = vec![
        vec![128u8, 128u8, 128u8],
        vec![150u8, 250u8, 250u8],
        vec![150u8, 150u8, 150u8],
        vec![16u8, 32u8, 48u8],
        vec![64u8, 33u8, 0u8],
        vec![200u8, 200u8, 200u8],
        vec![64u8, 31u8, 0u8],
    ];

    prepare_vec(&mut colors[..], 3);
    println!("{:?}", colors);
}

fn prepare_vec(mut vec: &mut [Vec<u8>], dimension: u8) {
    fn sort_split(v: &mut [Vec<u8>], current_dim: u8, max_dim: u8) {
        if v.len() == 1 {
            return; 
        }

        // We are sorting all the way here, but all we really need to do is
        // find the median element and partition around it, like in quicksort.
        // This could be faster, but for now this is plenty fast enough as we
        // only pay this cost when constructing the tree and not when querying.
        v.sort_unstable_by_key(|k| k[current_dim as usize]);
        let middle = v.len() / 2;
        let next_dim = (current_dim + 1) % max_dim;
        sort_split(&mut v[..middle], next_dim, max_dim);
        sort_split(&mut v[middle..], next_dim, max_dim);
    }
    sort_split(&mut vec[..], 0, dimension - 1);
}
