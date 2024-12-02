use std::fs;

fn main(){
    let contents = fs::read_to_string("./input.txt")
        .expect("should be able to read input file");

    
    let mut j: i32 = 0;
    let mut l1: Vec<i32> = Vec::new();
    let mut l2: Vec<i32> = Vec::new();
    for i in contents.split_whitespace(){
        if j % 2 == 0 {
            l1.push(i.parse().unwrap());
        } else {
            l2.push(i.parse().unwrap());
        }
        j += 1;
    }
    
    l1.sort();
    l2.sort();

    let mut sum: i32 = 0;

    for index in 0..l1.len(){
        let l1_element: i32 = *l1.get(index).unwrap();
        let l2_element: i32 = *l2.get(index).unwrap();


        if l1_element > l2_element {
            sum += l1_element - l2_element;
        } else {
            sum += l2_element - l1_element;
        }
    }


    let mut ss: Vec<i32> = Vec::new();
    let mut i_sum: i32 = 0;
    for i in l1.iter_mut(){    
        for j in l2.iter_mut(){
            if *i == *j{
                i_sum += 1;
            } else if *j > *i {
                break;
            }
        }
        ss.push(i_sum * *i);
        i_sum = 0;
    }

    let ss_sum: i32 = ss.iter().sum();

    println!("part one sum: {}", sum);
    println!("part two sum: {}", ss_sum);
}
