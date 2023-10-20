pub fn test_vector(){
  let v: Vec<i32> = Vec::new();
  println!("v: {:?}", v);

  let v = vec![1, 2, 3];
  println!("v: {:?}", v);

  let mut v2: Vec<i32> = Vec::new();
  v2.push(5);
  v2.push(6);
  v2.push(7);
  println!("v2: {:?}", v2);

  let v3 = vec![1, 2, 3, 4, 5];
  let third: &i32 = &v3[2];
  println!("third: {}", third);

  let third: Option<&i32> = v.get(2);
  println!("third: {:?}", third);
  match third {
      Some(third) => println!("third: {}", third),
      None => println!("third: None"),
  }

  let v4 = vec![1, 2, 3, 4, 5];
  // let does_not_exist = &v4[100];
  // println!("does_not_exist: {}", does_not_exist);
  let does_not_exist = v.get(100);
  println!("does_not_exist: {:?}", does_not_exist);

  let mut v5 = vec![1, 2, 3, 4, 5];
  let first = &v5[0];
  // v5.push(6);
  println!("first: {}", first);

  let v6 = vec![100, 32, 57];
  for i in &v6 {
      println!("{}", i);
  }

  let mut v7 = vec![100, 32, 57];
  for i in &mut v7 {
      *i += 50;
  }
  println!("v7: {:?}", v7);

  enum SpreadsheetCell {
      Int(i32),
      Float(f64),
      Text(String),
  }

  let row = vec![
      SpreadsheetCell::Int(3),
      SpreadsheetCell::Float(10.12),
      SpreadsheetCell::Text(String::from("blue")),
  ];

  {
      let v8 = vec![1, 2, 3, 4];
      // do stuff with v
  }// <- v goes out of scope and is freed here
}