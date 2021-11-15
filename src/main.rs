// -*- mode: Rustic; compile-command: "cargo run" -*-
pub mod renee;
pub mod natural_number;
mod lc;

use std::io::BufWriter;
use std::fs::File;
use std::io::Write;

use lc::Lc;
use renee::{U1, U4};
use natural_number::NaturalNumber;
use renee::Candidate;
fn main() {


    println!("Lc(2,2) = {}", Lc::new(2,2).and_list());
    println!("Lc(3,2) = {}", Lc::new(3,2).and_list());
    println!("Lc(4,3) = {}", Lc::new(4,3).and_list());

    let mut lc41 = Lc::new(2,4);
    let list = lc41.list();
    println!("Lc(2,4) expanded: {:?}", list);

    let p = 10;
    let N = 2i32.pow(p) as u32;

    let u4: U4<u32> = U4::new(N);

    //let saute = 1024*32;


    let arc = 3;


    //for i in 0..N/saute {

    //    if i > barc && i < earc {
    //    println!("\x1b[1m{} {} {}\x1b[0m", i, saute*i, lu4[(saute*i) as usize]);
    //    }
    //    else {
    //    println!("{} {} {}", i, saute*i, lu4[(saute*i) as usize]);

    //    }
    //}


    for i in 3..p {
        let lu1 = u4.lu(i);
        println!("\x1b[1marc {}:\x1b[0m\n{}", i, lu1);
        println!("{}", lu1.sauts());
    }


    //let mut writer = BufWriter::new(File::create("renee.txt").expect("coudln't open outfile"));

    //for i in 0..N {
    //    writeln!(writer, "{:3} {:7.4} {:7.4} {:4} {:4}", i,
    //    lu1[i] as f32 / i as f32,
    //    lu4[i] as f32 / i as f32,
    //    lu1[i], lu4[i]
    //);
    //}
}

fn make_fake_lu(arc: u32) -> Vec<u32> {
    vec![]
}

fn make_batr(N: usize) -> Vec<usize> {
    let mut bat: Vec<usize> = Vec::with_capacity(N);
    bat.push(1);
    bat.push(1);
    bat.push(1);

    for n in 3..N+1 {
        let b = bat[bat[n-1]] + bat[n-bat[n-1]];
        bat.push(b);
    }
    bat
}

fn make_seq(N: u32) -> Vec<usize> {
    let mut suite = Vec::with_capacity(N as usize);

    for n in 2..N+2 {
        suite.push(initial_problem(n));
    }
    suite
}

fn initial_problem(n: u32) -> usize {
    let mut deck = (1..n).collect::<Vec<u32>>();
    //println!("{:?}", deck);

    let mut first = 0;
    let mut table = Vec::with_capacity(2*n as usize);

    loop {
        deck.push(deck[first]);
        first += 1;
        if put_on_table(&mut deck, &mut table, first) {
            return deck.len() - first;
        }
        first += 1;

        //println!("table: {:?}", table);
        //if first == deck.len() { break; }
    }
}

fn batrachian(n: u32) -> u32 {
    if n < 3 { 1 }
    else {
        batrachian(batrachian(n-1)) + batrachian(n - batrachian(n-1))
    }
}

fn put_on_table(deck: &mut Vec<u32>, table: &mut Vec<u32>, first: usize) -> bool{
    let el = deck[first];
    table.push(el);
    if el == 1 {  true}
    else { false}
}

    #[cfg(test)]
mod test {
    use crate::make_seq;

    use super::initial_problem;

    #[test]
    fn test_initial() {
        let rsuite = vec![
            1,1,2,1,3,2,4,1,5,3,6,2,7,4,8,1,9,5,10,3,11,6,12,2,13
        ];

        assert_eq!(
            make_seq(rsuite.len() as u32),
            rsuite
        )        ;
    }
}
