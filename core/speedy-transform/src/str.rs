pub trait StringExtend {
  fn compare_handle(&self) -> std::string::String;
}

impl StringExtend for String {
  fn compare_handle(&self) -> std::string::String {
    self
      .chars()
      .filter(|c| !c.is_whitespace())
      .collect()
  }
}
