pub fn run() {
    println!("********* API Design Examples (Slices) *********");

    let v = vec![10, 20, 30];
    vector_borrow(&v);
    vector_only(v);

    let v = vec![10, 20, 30];
    vector_or_slice(&v);
    vector_or_slice(&v[2..]);

    let array : [i32; 3] = [99, 22, 44];
    //vector_only(array);       // Won't compile.
    //vector_borrow(&array);    // Won't compile.
    vector_or_slice(&array);


    let mut array : [i32; 3] = [99, 22, 44];
    mutable_vector_or_slice(&mut array);
}

fn vector_only(v: Vec<i32>) {
}

fn vector_borrow(v: &Vec<i32>) {
}

fn vector_or_slice(v: &[i32]) {
    // Guideline: take slices, not vectors.
    // v[0] = 3; Won't compile, immutable.
}

fn mutable_vector_or_slice(v: &mut [i32]) {
    // Guideline: take slices, not vectors.
    v[0] = 3;
}
