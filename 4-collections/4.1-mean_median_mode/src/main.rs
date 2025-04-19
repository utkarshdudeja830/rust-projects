// use std::collections::HashMap;

fn mean(vec: &Vec<i32>)-> f32{
    let mut sum = 0;

    for i in 0..vec.len(){
        sum += vec[i];
    }
    sum as f32 / vec.len() as f32
}

fn median(vec: &mut Vec<i32>)-> f32{
    vec.sort();

    let n = vec.len();

    if n == 0{
        return 0.0;
    }

    if n % 2 == 0 {

        let mid1 = vec[(n-1)/2];
        let mid2 = vec[n/2];
        (mid1 + mid2) as f32/2.0

    }else{
        return vec[n/2] as f32
    }
}

// fn mode(vec: &Vec<i32>)-> f32{
    
// }


fn main() {
    let mut  vec = vec![1,2,3,4];
    let vec_mean = mean(&vec);
    println!("The mean is {}.",vec_mean);

    let vec_median = median(&mut vec);
    println!("The median is {}.",vec_median);

    // let mut map = HashMap::New();

    // let vec_median = median(&mut vec);
    // println!("The median is {}.",vec_median);
}