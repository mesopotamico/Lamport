use rand::Rng;

pub fn rng_array(size: usize) -> Vec<i32> {

    let mut rng = rand::thread_rng();
    let mut disorder: Vec<i32> = (0..size)
        .map(|_| rng.gen_range(1..=100)) 
        .collect();

    disorder

}
