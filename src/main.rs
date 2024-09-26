use rand::Rng;
mod quick_sort;
use quick_sort::quick_sort;

fn main() {
    let init2: usize = 0;
    let end2: usize = 1000;

    let mut rng = rand::thread_rng();
    let mut disorder: Vec<i32> = (0..=end2)
        .map(|_| rng.gen_range(1..=100)) 
        .collect();

    println!("{:?}", disorder);
    quick_sort(&mut disorder, init2, end2);
    println!("Este es ordenado");
    println!("{:?}", disorder);
}

