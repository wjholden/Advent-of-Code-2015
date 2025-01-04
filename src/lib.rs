#[derive(Debug)]
pub struct ConstantSum {
    length: usize,
    total: i32,
    current: i32,
    child: Option<Box<ConstantSum>>,
}

impl ConstantSum {
    fn new(length: usize, total: i32) -> ConstantSum {
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
        println!("{self:?}");
        match self.child {
            None => { // base case: no child, one fixed value.
                if self.current < self.total {
                    self.current = self.total;
                    return Some(vec![self.total])
                } else {
                    return None
                }
            },
            Some(ref mut child) => { // recursive case: iterate over child first, then self.
                match child.next() {
                    Some(mut v) => {
                        let mut u = vec![self.current];
                        u.append(&mut v);
                        Some(u)
                        //v.push(self.current);
                        //Some(v)
                    },
                    None => {
                        self.current += 1;
                        if self.current > self.total {
                            return None
                        } else {
                            self.child = Some(Box::new(ConstantSum::new(self.length - 1, self.total - self.current)));
                            return self.next()
                        }
                    }
                }
            }
        }
    }
}

mod test {
    use std::assert_eq;

    use super::*;

    #[test]
    fn constant_sum() {
        let v: Vec<Vec<i32>> = ConstantSum::new(3, 2).collect();
        println!("{v:?}");
    }
 
    #[test]
    fn test2() {
        //assert_eq!(part2(SAMPLE), 0)
    }   
}