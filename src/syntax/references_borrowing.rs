pub fn test_references_borrowing() {
    let mut x: i32 = 10;
    let ref_x = &mut x;
    *ref_x = 20;
    // println!("x: {x}");
    println!("ref_x: {ref_x}");
    let mut x: i32 = 10;
    let ref_x = &mut x;
    *ref_x = 20;
    // println!("x: {}", x);
    println!("ref_x: {}", ref_x);

    //error: borrowed value does not live long enough
    let ref_x: &i32;
    {
        let xx: i32 = 10;
        // ref_x = &xx;
    }
    // println!("ref_x: {ref_x}");

    let ref_x: &i32;
    {
        let xx: i32 = 10;
        ref_x = &xx;
        println!("ref_x: {}", ref_x);
    }

    let xx: i32 = 10;
    {
        let ref_x: &i32;
        ref_x = &xx;
        println!("ref_x: {}", ref_x);
    }
}
