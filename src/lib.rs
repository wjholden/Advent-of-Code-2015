#[derive(Debug)]
/// In day 15 we need all length-4 vectors of nonnegative integers
/// and sum of exactly 100. This is trickier than it sounds.
/// If Rust had a yield keyword (and apparently it sorta does?)
/// then this would have been easy (see https://stackoverflow.com/a/29171375/5459668).
/// This was pretty miserable to write. If I had to do it again,
/// I might have used a Go-style channel for an intuitive, recursive solution.
pub struct ConstantSum {
    length: usize,
    total: i32,
    current: i32,
    child: Option<Box<ConstantSum>>,
}

impl ConstantSum {
    pub fn new(length: usize, total: i32) -> ConstantSum {
        let child = if length == 1 {
            None
        } else {
            Some(Box::new(ConstantSum::new(length - 1, total)))
        };
        ConstantSum {
            length,
            total,
            current: 0,
            child
        }
    }
}

impl Iterator for ConstantSum {
    type Item = Vec<i32>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.length == 1 {
            // We need to know if we've read our one entry or not.
            if self.current > self.total {
                return None
            } else {
                // Intentionally overshoot the total so that we can return zeros.
                self.current = self.total + 1;
                return Some(vec![self.total])
            }
        }

        if self.current <= self.total {
            match self.child {
                None => unreachable!(),
                Some(ref mut child) => {
                    match child.next() {
                        None => {
                            self.current += 1;
                            if self.current > self.total {
                                None
                            } else {
                                self.child = Some(Box::new(ConstantSum::new(self.length - 1, self.total - self.current)));
                                self.next()
                            }
                        },
                        Some(mut v) => {
                            v.push(self.current);
                            Some(v)
                        }
                    }
                }
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::Rng;

    #[test]
    fn constant_sum_3_2() {
        // https://stackoverflow.com/a/29171375/5459668
        let mut cs = ConstantSum::new(3,2);
        assert_eq!(cs.next(), Some(vec![2,0,0]));
        assert_eq!(cs.next(), Some(vec![1,1,0]));
        assert_eq!(cs.next(), Some(vec![0,2,0]));
        assert_eq!(cs.next(), Some(vec![1,0,1]));
        assert_eq!(cs.next(), Some(vec![0,1,1]));
        assert_eq!(cs.next(), Some(vec![0,0,2]));
        assert_eq!(cs.next(), None);
    } 

    #[test]
    fn constant_sum_4_6() {
        assert_eq!(ConstantSum::new(4,6).count(), 84)
    }

    #[test]
    fn constant_sum_4_100() {
        assert_eq!(ConstantSum::new(4,100).count(), 176851)
    }

    #[test]
    fn random_sum_test() {
        let mut rng = rand::thread_rng();
        let length = rng.gen_range(1usize..4);
        let total = rng.gen_range(1i32..100);
        // Generate a random total to make, then verify
        // that our generator actually adds up to that.
        for x in ConstantSum::new(length, total) {
            assert_eq!(x.len(), length);
            assert_eq!(x.into_iter().sum::<i32>(), total);
        }
    }
}