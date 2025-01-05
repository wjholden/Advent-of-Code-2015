/// In day 15 we need all length-4 vectors of nonnegative integers
/// and sum of exactly 100. This is trickier than it sounds.
/// If Rust had a yield keyword (and apparently it sorta does?)
/// then this would have been easy (see https://stackoverflow.com/a/29171375/5459668).
/// This was pretty miserable to write. If I had to do it again,
/// I might have used a Go-style channel for an intuitive, recursive solution.
pub struct ConstantSumSeq {
    length: usize,
    total: i32,
    current: i32,
    child: Option<Box<ConstantSumSeq>>,
}

impl ConstantSumSeq {
    pub fn new(length: usize, total: i32) -> ConstantSumSeq {
        let child = if length == 1 {
            None
        } else {
            Some(Box::new(ConstantSumSeq::new(length - 1, total)))
        };
        ConstantSumSeq {
            length,
            total,
            current: 0,
            child
        }
    }
}

impl Iterator for ConstantSumSeq {
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
                                self.child = Some(Box::new(ConstantSumSeq::new(self.length - 1, self.total - self.current)));
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

    /// This test is derived from https://stackoverflow.com/a/29171375/5459668.
    /// 
    /// Our Rust program produces the same outputs as the Python
    /// ```no_run
    /// def get_tuples(length, total):
    ///     if length == 1:
    ///         yield (total,)
    ///         return
    /// 
    ///     for i in range(total + 1):
    ///         for t in get_tuples(length - 1, total - i):
    ///             yield (i,) + t
    /// 
    /// list(get_tuples(3, 2))
    /// ```
    /// but in a different order.
    #[test]
    fn constant_sum_3_2() {
        // https://stackoverflow.com/a/29171375/5459668
        let mut cs = ConstantSumSeq::new(3,2);
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
        assert_eq!(ConstantSumSeq::new(4,6).count(), 84)
    }

    #[test]
    fn constant_sum_4_100() {
        assert_eq!(ConstantSumSeq::new(4,100).count(), 176851)
    }

    /// Generate a random total to make, then verify that our generator actually
    /// adds up to that. This doesn't look at the values, just their lengths
    /// and sums.
    #[test]
    fn random_sum_test() {
        let mut rng = rand::thread_rng();
        let length = rng.gen_range(1usize..4);
        let total = rng.gen_range(1i32..100);
        for x in ConstantSumSeq::new(length, total) {
            assert_eq!(x.len(), length);
            assert_eq!(x.into_iter().sum::<i32>(), total);
        }
    }

    /// This test is derived from https://adventofcode.com/2015/day/15.
    /// 
    /// Using Mathematica
    /// ```no_run
    /// In[1]:= Solve[{500==8x+3y,100==x+y},{x,y},NonNegativeIntegers]
    /// Out[1]= {{x -> 40, y -> 60}}
    /// ```
    /// we find only one candidate solution for
    /// ```no_run
    /// Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
    /// Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3
    /// ```
    /// where we have 100 total units of each ingredient and 500 total calories.
    #[test]
    fn aoc_2015_day15_part2_example() {
        let mut cs = ConstantSumSeq::new(2, 100).filter(|v| v[0]*8+v[1]*3 == 500);
        assert_eq!(cs.next(), Some(vec![40,60]));
        assert_eq!(cs.next(), None);
    }
}