fn are_you_playing_banjo(name: &str) -> String {
  let starts_with_r = name.to_lowercase().starts_with("r");
   
   match starts_with_r {
       true => format!("{} plays banjo", name),
       false => format!("{} does not play banjo", name) 
   }
}