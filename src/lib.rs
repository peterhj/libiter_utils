pub fn argmax<T, Iter>(xs: Iter) -> Option<usize> where T: Copy + Ord, Iter: Iterator<Item=T> {
  let mut argmax: Option<(usize, T)> = None;
  for (i, x) in xs.enumerate() {
    match argmax {
      None => {
        argmax = Some((i, x));
      }
      Some((_, y)) => {
        if y < x {
          argmax = Some((i, x));
        }
      }
    }
  }
  argmax.map(|(i, _)| i)
}

#[test]
fn test_argmax() {
  let xs: Vec<usize> = vec![0, 3, 2, 1];
  let i1 = argmax(xs.iter());
  let i2 = argmax(xs.iter().map(|&x| x));
  assert_eq!(Some(1), i1);
  assert_eq!(Some(1), i2);
  assert_eq!(i1, i2);
}
