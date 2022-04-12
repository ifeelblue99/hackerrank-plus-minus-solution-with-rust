fn main() {
  let mut data = [1, 1, 1, -1, 0, 0, -1, 0];

  if let Some(res) = get_ratio(&mut data) {
    println!("ratio: {:?}", res)
  } else {
    println!("something bad happened...")
  }
}
fn get_ratio(arr: &mut [i8]) -> Option<[f32;3]> {
  arr.sort();
  let length = arr.len() as f32;
  let mut counts = [0i8; 3];
  let mut result = [0.0f32; 3];
  
  for el in arr.iter() {
    match el {
      -1 => counts[0] += 1,
      0 => counts[1] += 1,
      1 => counts[2] += 1,
      _=> println!("not exists..."),
    }
  }
  let mut indx:usize = 0;
  result = loop {
    result[indx] = counts[indx] as f32/length; 
    indx = indx + 1;
    
    if indx>2 {
      break result;
    }
  };
  Some(result)
}
