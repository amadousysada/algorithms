
fn bubble_sort(v: &mut Vec<u32>) -> &mut Vec<u32> {
    let n = v.len()-1;
    for _i in 0..n {
        for j in 0..n{
            if v[j] > v[j+1]{
                v.swap(j, j+1)
            }
        }
    }
    v
}


fn main() {
    let mut v = vec![0, 3, 1, 5, 2];
    println!("Sorted vector {:?}", bubble_sort(&mut v))
}
