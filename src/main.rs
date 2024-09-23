fn main() {
    let mut vector: Vec<i32> = vec![5,3,1,7,3,2,6,4];

    let mut init: usize = 0;
    let mut end: usize = 7;
    let pivot = place_pivot(vector, init, end);
    println!("{:?}", pivot);
}


fn place_pivot(mut vector: Vec<i32>, mut start:usize , mut end: usize) -> usize{

    let mut pivot: usize = 0; 
    while start < end {
        while vector[end] >= vector[start] && start < end{
            end = end - 1;
        }
        let mut _old: i32 = vector[end]; 
        vector[end] = vector[start];
        vector[start] = vector[end];

        while vector[start] <= vector[end] && start < end{
            start = start + 1;
        }
        let mut _old: i32 = vector[end]; 
        vector[end] = vector[start];
        vector[start] = vector[end];
    }

    start



}



fn quick_sort(mut vector: Vec<i32>, mut start:usize , mut end: usize) {


}
