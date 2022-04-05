fn even_or_odd(i: i32) -> &'static str {
  let reimender = i % 2;
  
  let is_even_or_odd = match reimender {
      0 => "Even",
      _ => "Odd"
  };
  
  is_even_or_odd
}