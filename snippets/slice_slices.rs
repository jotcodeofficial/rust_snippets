/// title: slice slices
/// tags: slice, slices,

fn slice_slices() {
    let numbers: [u8; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let cut_1 = &numbers[..3];
    let cut_2 = &numbers[4..7];
    let cut_3 = &numbers[4..];
    println!(
        "cut 1: {:#?}\ncut 2: {:#?}\ncut 3: {:#?}",
        cut_1, cut_2, cut_3
    );
}
