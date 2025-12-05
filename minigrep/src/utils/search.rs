pub fn search_str<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let query = query.to_lowercase();

  contents.lines().filter(|line| {
    line.to_lowercase().contains(&query)
  }).collect()
}
