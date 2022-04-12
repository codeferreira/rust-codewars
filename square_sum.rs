fn square_sum(vec: Vec<i32>) -> i32 {
  let mut sum = 0;
  
  if vec.len() > 0 {
      for number in vec {
          sum = sum + (number.pow(2))
      }
  }
  
  sum
}