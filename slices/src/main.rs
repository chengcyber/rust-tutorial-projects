fn main() {
  let s = String::from("Hello world");

  let word = first_word(&s[..]);

  let s2 = "hello world";

  let word = first_word(&s2[..]);

  let word = first_word(s2);
}

fn first_word(s: &str) -> &str {

  let bytes = s.as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      return &s[0..i];
    }
  }

  &s[..]
}
