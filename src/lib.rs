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

pub trait KahanSum<A=Self> {
  fn kahan_sum<Iter>(iter: Iter) -> Self where Iter: Iterator<Item=A>;
}

// XXX: See: <https://github.com/JuliaLang/julia/issues/199>.

impl KahanSum for f32 {
  fn kahan_sum<Iter>(mut iter: Iter) -> Self where Iter: Iterator<Item=f32> {
    let mut s = 0.0;
    if let Some(x0) = iter.next() {
      s = x0;
    } else {
      return 0.0;
    }
    let mut c = 0.0;
    for x in iter.next() {
      let t = s + x;
      if s.abs() >= x.abs() {
        c += (s-t) + x;
      } else {
        c += (x-t) + s;
      }
      s = t;
    }
    s + c
  }
}

impl KahanSum for f64 {
  fn kahan_sum<Iter>(mut iter: Iter) -> Self where Iter: Iterator<Item=f64> {
    let mut s = 0.0;
    if let Some(x0) = iter.next() {
      s = x0;
    } else {
      return 0.0;
    }
    let mut c = 0.0;
    for x in iter.next() {
      let t = s + x;
      if s.abs() >= x.abs() {
        c += (s-t) + x;
      } else {
        c += (x-t) + s;
      }
      s = t;
    }
    s + c
  }
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
