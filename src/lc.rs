use std::fmt::Display;

struct LcFlat(u32, u32);

impl LcFlat {
    pub fn list(&self) -> Vec<u32> {
        if self.0 == 0 {
            return vec![1, 0];
        }
        if self.0 >= 1 && self.1 == 1 {
            return vec![self.0, 1];
        }
        if self.0 >= self.1 {
            let mut ret_vec = Vec::with_capacity(2u64.pow(self.0) as usize);
            for n in (2..=self.0).rev() {
                ret_vec.extend(LcFlat(n, self.1 - 1).list());
            }
            //ret_vec.extend(Lc(2,self.1-1).list());
            ret_vec.extend(LcFlat(1, self.1).list());
            return ret_vec;
        }
        if self.0 < self.1 {
            let mut revlist = LcFlat(self.1, self.0).list();
            revlist.reverse();
            return revlist;
        } else {
            return vec![];
        }
    }
}

enum LcListItem {
    Value(u32),
    Lc(Lc),
}

impl LcListItem {
    pub fn lc(n: u32, k: u32) -> Self {
        LcListItem::Lc(Lc::new(n, k))
    }
    pub fn value(v: u32) -> Self {
        LcListItem::Value(v)
    }
    pub fn reverse(&mut self) {
        if let LcListItem::Lc(lc) = self {
            lc.reverse();
        }
    }
}

impl Display for LcListItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LcListItem::Value(v) => write!(f, "{}", v),
            LcListItem::Lc(lc) => write!(f, "{}", lc),
        }
    }
}

pub struct Lc {
    n: u32,
    k: u32,
    list: Vec<LcListItem>,
}

impl Lc {
    pub fn new(n: u32, k: u32) -> Self {
        Self {
            n,
            k,
            list: Vec::new(),
        }
    }
    pub fn calc_list(&mut self) {
        if self.k == 0 {
            self.list = vec![LcListItem::Value(1), LcListItem::Value(0)];
        }
        if self.n >= 1 && self.k == 1 {
            self.list = vec![LcListItem::Value(self.n), LcListItem::Value(1)];
        }
        if self.n >= self.k  && self.k > 1{
            self.list = Vec::with_capacity(self.n as usize);
            for n in (2..=self.n).rev() {
                self.list.push(LcListItem::lc(n, self.k - 1));
            }
            //ret_vec.extend(Lc(2,self.1-1).list());
            self.list.push(LcListItem::lc(1, self.k));
        }
        if self.n < self.k {
            let mut rev_lc = Lc::new(self.k, self.n)
            .and_list();
            self.list = rev_lc.list;
        }
    }

    /// calcs list and returns self
    pub fn and_list(mut self) -> Self {
        self.calc_list();
        self
    }

    pub fn and_reversed(mut self) -> Self {
        self.reverse();
        self
    }

    pub fn reverse(&mut self) {
        self.list.reverse();
        //for x in self.list.iter_mut() {
        //    x.reverse();
        //}
    }

    pub fn arc(&self) -> u32 {
        self.n.max(self.k)
    }

    pub fn list(&mut self) -> Vec<u32> {
        if self.list.len() == 0 {
            self.calc_list();
        }
        //println!("expand self: {}", self);

        let mut list: Vec<u32> = Vec::with_capacity(2u64.pow(self.arc()) as usize);

        for sublc in self.list.iter_mut() {
            match sublc {
                LcListItem::Lc(lc) => list.extend(lc.list()),
                &mut LcListItem::Value(value) => list.push(value),
            };
        }

        if self.n < self.k {
            list.reverse();
        }
        list
    }
}

impl Display for Lc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.list.len() == 0 {
            write!(f, "Lc({},{})", self.n, self.k)
        } else {
            write!(f, "({}", self.list[0])?;
            for x in self.list.iter().skip(1) {
                write!(f, ", {}", x)?;
            }
            write!(f, ")")
        }
    }
}

#[cfg(test)]
mod lc_tests {
    use super::*;
    #[test]
    fn test_lc_2_2() {
        assert_eq!(Lc::new(2, 2).list(), vec![2, 1, 1, 2]);
    }

    #[test]
    fn test_lc_2_3() {
        assert_eq!(Lc::new(2, 3).list(), vec![2, 1, 1, 2, 1, 3]);
    }

    #[test]
    fn test_lc_3_3() {
        assert_eq!(
            Lc::new(3, 3).list(),
            vec![3, 1, 2, 1, 1, 2, 2, 1, 1, 2, 1, 3]
        );
    }

    #[test]
    fn test_lc_1_4() {
        assert_eq!(Lc::new(1, 4).list(), vec![1, 4]);
    }

    #[test]
    fn test_lc_4_1() {
        assert_eq!(Lc::new(4, 1).list(), vec![4,1]);
    }

    #[test]
    fn test_lc_2_4() {
        assert_eq!(Lc::new(2, 4).list(), vec![2, 1, 1, 2, 1, 3, 1, 4]);
    }

    #[test]
    fn test_lc_3_4() {
        assert_eq!(
            Lc::new(3, 4).list(),
            vec![3, 1, 2, 1, 1, 2, 2, 1, 1, 2, 1, 3, 2, 1, 1, 2, 1, 3, 1, 4]
        );
    }
    #[test]
    fn test_lc_4_3() {
        let mut answer = 
            vec![3, 1, 2, 1, 1, 2, 2, 1, 1, 2, 1, 3, 2, 1, 1, 2, 1, 3, 1, 4];
        answer.reverse();
        assert_eq!(
            Lc::new(4,3).list(),
            answer
        );
    }
}

#[cfg(test)]
mod lc_flat_tests {
    use super::*;
    #[test]
    fn test_lc_2_2() {
        assert_eq!(LcFlat(2, 2).list(), vec![2, 1, 1, 2]);
    }

    #[test]
    fn test_lc_2_3() {
        assert_eq!(LcFlat(2, 3).list(), vec![2, 1, 1, 2, 1, 3]);
    }

    #[test]
    fn test_lc_3_3() {
        assert_eq!(
            LcFlat(3, 3).list(),
            vec![3, 1, 2, 1, 1, 2, 2, 1, 1, 2, 1, 3]
        );
    }

    #[test]
    fn test_lc_1_4() {
        assert_eq!(LcFlat(1, 4).list(), vec![1, 4]);
    }

    #[test]
    fn test_lc_2_4() {
        assert_eq!(LcFlat(2, 4).list(), vec![2, 1, 1, 2, 1, 3, 1, 4]);
    }

    #[test]
    fn test_lc_3_4() {
        assert_eq!(
            LcFlat(3, 4).list(),
            vec![3, 1, 2, 1, 1, 2, 2, 1, 1, 2, 1, 3, 2, 1, 1, 2, 1, 3, 1, 4]
        );
    }
}
