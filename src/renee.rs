// -*- mode: Rustic; compile-command: "cargo run" -*-

use std::{fmt::Display, ops::Sub};

use crate::natural_number::NaturalNumber;

// -------------------------------------------------------------------------------------------------
// ---- Candidate ----------------------------------------------------------------------------------
// -------------------------------------------------------------------------------------------------

pub trait Candidate {
    fn position(&self, n: usize) -> usize;
    fn suite(&self, N: usize) -> Vec<usize>;
}

// -------------------------------------------------------------------------------------------------
// ---- Steps --------------------------------------------------------------------------------------
// -------------------------------------------------------------------------------------------------
pub struct Steps {
    pub steps: Vec<Step>,
    pub arc: u32
}

impl Steps {
    pub fn new(steps: Vec<Step>, arc: u32) -> Self {
        Self {
            steps,
            arc
        }
    }

}

impl<'a> Steps {
    pub fn sauts(&'a self) -> Sauts {
        Sauts::new(self)
    }
}

impl Display for Steps {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let half_len = self.steps.len() / 2;

        write!(f, "[")?;

        for v in self.steps.iter().take(half_len) {
            write!(f, "{}", v)?;
        }

        write!(f," ... ]")
    }
}

pub struct Sauts<'a> {
    steps: &'a Steps
}

impl<'a> Sauts<'a> {
    pub fn new(steps: &'a Steps) -> Self {
        Self{ steps }
    }
}

impl<'a> Display for Sauts<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;

        let iter2 = self.steps.steps.iter().step_by(2);

        for v in iter2{
            write!(f, "{} ", v)?;
        }

        writeln!(f,"]")


    }
}

pub struct Blocks<'a> {
    steps: &'a Steps
}



// -------------------------------------------------------------------------------------------------
// ---- Step ---------------------------------------------------------------------------------------
// -------------------------------------------------------------------------------------------------

#[derive(Clone, Copy)]
pub enum Step {
    Saut(u32),
    Repose(u32),
}

impl std::fmt::Display for Step {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Saut(arg0) => write!(f, "\x1b[32m{}\x1b[0m", arg0),
            Self::Repose(arg0) => write!(f, "{}", arg0),
        }
    }
}

impl Sub<u32> for Step {
    type Output = Step;

    fn sub(self, rhs: u32) -> Self::Output {
        match self {
            Step::Saut(a) => Step::Saut(a-rhs),
            Step::Repose(a) => Step::Repose(a-rhs),
        }
    }
}

// -------------------------------------------------------------------------------------------------
// ---- U4 -----------------------------------------------------------------------------------------
// -------------------------------------------------------------------------------------------------

pub struct U4<T> {
    pub crap: Vec<T>,
}

fn calc_crap<T>(n: T, bat: &[T]) -> T
where
    T: NaturalNumber,
{
    let second = n.to_usize() - bat[n.to_usize() - 1].to_usize();
    bat[bat[n.to_usize() - 1].to_usize()] + bat[second]
}

impl<T> U4<T>
where
    T: NaturalNumber,
{
    pub fn new(N: T) -> Self {
        let mut crap: Vec<T> = Vec::with_capacity(N.to_usize());

        crap.push(T::from_usize(0));
        crap.push(T::from_usize(1));
        crap.push(T::from_usize(1));

        for n in 3..N.to_usize() + 1 {
            crap.push(calc_crap(T::from_usize(n), &crap));
        }

        Self { crap }
    }

    pub fn fill(&mut self, N: T) {
        for i in self.crap.len() - 1..N.to_usize() {
            self.crap.push(calc_crap(T::from_usize(i), &self.crap));
        }
    }
}

impl<T: NaturalNumber + std::fmt::Display> U4<T> {
    pub fn lu(&self, arc: u32) -> Steps {
        let begin = 2u32.pow(arc);
        let end = 2u32.pow(arc+1);
        let mut v: Vec<Step> = vec![];

        let mut saut = true;
        let mut curr = 0;

        for i in begin..end {
            //println!("{}->{}", list[i], list[i+1]);
            if saut {
                if self.crap[i as usize] == self.crap[i as usize + 1] {
                    v.push(Step::Saut(curr));
                    saut = false;
                    curr = 1;
                } else {
                    curr += 1;
                }
            } else {
                if self.crap[i as usize] == self.crap[i as usize + 1] {
                    curr += 1;
                } else {
                    v.push(Step::Repose(curr));
                    saut = true;
                    curr = 1;
                }
            }
        }
        v.push(if saut {
            Step::Saut(curr)
        } else {
            Step::Repose(curr)
        });
        //v[1..].into_iter().map(|x| *x).collect::<Vec<_>>()
        let mut v2 = Vec::with_capacity(v.len()+4);
        v2.push(Step::Saut(1));
        v2.push(Step::Repose(0));
        v2.push(v[0]-1);
        v2.push(v[1]);
        for x in v[2..v.len()-2].iter() {
            v2.push(*x);
        }
        v2.push(v[v.len()-2]);
        v2.push(v[v.len()-1]-1);
        v2.push(Step::Saut(0));
        v2.push(Step::Repose(1));
        Steps::new(v2, arc)
    }
}

//impl Candidate for U4 {
//    fn position(&self, n: usize) -> usize {
//        self.crap[n]
//    }
//
//    fn suite(&self, N: usize) -> Vec<usize> {
//        self.crap[0..N].to_vec()
//    }
//}

pub struct U1;

impl Candidate for U1 {
    fn position(&self, n: usize) -> usize {
        (n + 1) / 2
    }

    fn suite(&self, N: usize) -> Vec<usize> {
        let mut lu1 = Vec::with_capacity(N);

        for n in 0..N {
            lu1.push(self.position(n));
        }

        lu1
    }
}
