#![allow(dead_code)] // Ignora advertencias sobre funciones no utilizadas
use rand::Rng;


fn main() {
    let init2: usize = 0;
    let end2: usize = 10;

    let mut rng = rand::thread_rng();
    let mut disorder: Vec<i32> = (0..end2 + 1)
        .map(|_| rng.gen_range(1..=100)) 
        .collect();

    println!("{:?}", disorder);
    quick_sort(&mut disorder, init2, end2);
    println!("Este es ordenado");
    println!("{:?}", disorder);
}

fn place_pivot(vector: &mut Vec<i32>, mut start:usize , mut end: usize) -> usize{

    while start < end {
        while vector[end] >= vector[start] && start < end{
            end = end - 1;
        }
        let mut _old: i32 = vector[end]; 
        vector[end] = vector[start];
        vector[start] = _old;

        while vector[start] <= vector[end] && start < end{
            start = start + 1;
        }
        let mut _old: i32 = vector[end]; 
        vector[end] = vector[start];
        vector[start] = _old;
    }
    start
}
// remove the mut in the start and the finish, but in the place_pivot is needed
fn quick_sort(mut vector: &mut Vec<i32>, start:usize , end: usize) {
    if start < end {
        let right_pos = place_pivot(&mut vector, start, end);
        if right_pos != 0 {
            quick_sort(&mut vector, start, right_pos - 1);
        } 
        quick_sort(&mut vector,right_pos + 1, end);
        
    }

}
