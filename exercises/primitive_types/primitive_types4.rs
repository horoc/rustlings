// primitive_types4.rs
// Get a slice out of Array a where the ??? is so that the test passes.
// Execute `rustlings hint primitive_types4` or use the `hint` watch subcommand for a hint.

// I AM NOT DONE

#[test]
fn slice_out_of_array() {
    let a = [1, 2, 3, 4, 5];

    // slice 是动态类型，只能在堆上，只能通过引用访问
    let nice_slice = &a[1..5]; 

    assert_eq!([2, 3, 4], nice_slice)
}
