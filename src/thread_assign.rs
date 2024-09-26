pub fn thread_assign(vector: Vec<i32>, threads: usize) -> Vec<Vec<i32>> {
    //scores.insert(1,2);
    let heap: usize = vector.len();
 
    let mut thread_assign = Vec::new();
    for _ in 0..threads {
        thread_assign.push(Vec::new())
    }

    let mut x: usize = 0;
    for i in 0..heap {
        if x == threads{
            x = 0;
        }
        thread_assign[x].push(vector[i]);
        x = x+1;
    }

    thread_assign

}

