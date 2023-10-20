pub fn test_slice(){
  let mut a: [i32; 6] = [10, 20, 30, 40, 50, 60];
  println!("a: {a:?}");

  a[3] = 33;
  let s: &[i32] = &a[1..4];
  //a[3] = 44;
  println!("s: {s:?}");
}