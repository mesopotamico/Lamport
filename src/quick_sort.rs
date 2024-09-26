pub fn place_pivot(vector: &mut Vec<i32>, mut start:usize , mut end: usize) -> usize{

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
pub fn quick_sort(mut vector: &mut Vec<i32>, start:usize , end: usize) {

    if start < end {
        let right_pos = place_pivot(&mut vector, start, end);
        if right_pos != 0 {
            quick_sort(&mut vector, start, right_pos - 1);
        } 
        quick_sort(&mut vector,right_pos + 1, end);
        
    }

}
