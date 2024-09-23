use rand::Rng;

fn main() {

    let mut end2: usize = 20;
    let mut rng = rand::thread_rng();
    let mut disorder: Vec<i32> = (0..end2 + 1)
        .map(|_| rng.gen_range(1..=100)) 
        .collect();

    let mut init2: usize = 0;
    println!("{:?}", disorder);
    quick_sort(&mut disorder, init2, end2);
    println!("{:?}", disorder);
}


fn place_pivot(mut vector: &mut Vec<i32>, mut start:usize , mut end: usize) -> usize{

    let mut pivot: usize = 0; 
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

fn quick_sort(mut vector: &mut Vec<i32>, mut start:usize , mut end: usize) {
    if start < end {
        let right_pos = place_pivot(&mut vector, start, end);
        if right_pos != 0 {
            quick_sort(&mut vector, start, right_pos - 1);
        } 
        quick_sort(&mut vector,right_pos + 1, end);
        
    }

}
