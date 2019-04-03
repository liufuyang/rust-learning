fn main() {
    assert_eq!(24, std::mem::size_of::<Vec<i32>>());
    assert_eq!(8, std::mem::size_of::<&Vec<i32>>());
    assert_eq!(16, std::mem::size_of::<&[i32]>());

    assert_eq!(24, std::mem::size_of::<String>());
    assert_eq!(8, std::mem::size_of::<&String>());
    assert_eq!(16, std::mem::size_of::<&str>());

    let mut s = String::with_capacity(10); // or you can do `"山西人都挺好".to_string();` to make the same String.
    assert_eq!(0, std::mem::size_of_val(&*s));

    s.push_str("山西人都挺好"); // 6 Chinese character
    let s_ref = &s; // &String
    let s_slice = &s[..]; // &str

    println!("s: {}", &s);
    println!("s.len(): {}", s.len()); // 18, as each Chinese character is a 3-byte UTF-8 character
    println!("s.chars().count(): {}", s.chars().count());
    println!("s.capacity: {}", s.capacity()); // 20, as init by 10, then increase to 10x2

    assert_eq!(24, std::mem::size_of_val(&s)); // [address, capacity, length]
    assert_eq!(8, std::mem::size_of_val(&s_ref)); // [address]
    assert_eq!(16, std::mem::size_of_val(&s_slice)); // [address, length], slice is a fat pointer
    assert_eq!(18, std::mem::size_of_val(&*s)); // each Chinese character is a 3-byte UTF-8 character

    let v = vec![1u8, 2, 3, 4, 5, 6, 7]; // on heap

    let va = &v;
    let vb = &v[..];
    println!("v.len(): {}", v.len());
    println!("v.capacity(): {}", v.capacity());
    println!("va add: {:p}", va);
    println!("vb add: {:p}", vb); // they are different of course

    assert_eq!(24, std::mem::size_of_val(&v));
    assert_eq!(8, std::mem::size_of_val(&va));
    assert_eq!(16, std::mem::size_of_val(&vb)); // fat pointer as slice
    assert_eq!(24, std::mem::size_of_val(va)); // same as &v

    assert_eq!(7, std::mem::size_of_val(vb)); // should be 7byte as there is 7 u8
    assert_eq!(7, std::mem::size_of_val(&*v)); // should be 7byte as there is 7 u8

    let v_stack = [1u64, 2, 3, 4, 5, 6, 7]; // on stack, u64 is u8*8
    assert_eq!(7 * 8, std::mem::size_of_val(&v_stack)); // works the same as
}
