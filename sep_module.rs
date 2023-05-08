pub fn delete_nones(all_info: &Vec<(usize,usize,usize)>) -> Vec<(usize,usize,usize)> {

    let all_info_no_zero: Vec<_> = all_info.iter().filter(|&(c, _, _)| c != &0usize).map(|&a| a).collect();
    
    return all_info_no_zero;

}


pub fn find_average(all_info_no_zero: &Vec<(usize,usize,usize)>) -> f64 {
    
    let mut sum = 0;
    for i in all_info_no_zero.iter() {

        sum += i.0

    }

    let avg = (sum / all_info_no_zero.len()) as f64;
    return avg

}



pub fn six_deg_check(all_info: &Vec<(usize,usize,usize)>) {

    let mut num_above = 0;

    for i in all_info.iter() {
        if i.0 > 6 {
            println!("Going from {} to {} will be more than 6 degrees. It has a distance of {}",i.1,i.2,i.0);
            num_above += 1

        }
    }

    println!("there are {:?} pairs of nodes that have more than 6 degrees separating them", num_above);
}


pub fn highest_dist(all_info: &Vec<(usize,usize,usize)>)  {

    let mut highest_dist = (0,0,0);
    
    for i in all_info.iter() {
        if i.0 > highest_dist.0 {
            highest_dist = *i
        }
    }

    println!("the farthest distance between two nodes is {:?} between {} and {}", highest_dist.0, highest_dist.1, highest_dist.2);

}



pub fn find_median(mut all_info_no_zero: Vec<(usize,usize,usize)>) {

    let vec_len = all_info_no_zero.len();

    all_info_no_zero.sort_by_key(|f| f.0);

    if vec_len % 2 == 0 {
        let middle = vec_len / 2;
        let median = all_info_no_zero[middle].0;
        println!("the median distance is {:?}",median)
    }

    else {
        let lower_middle = (vec_len /2 ) as f64 - 0.5;
        let upper_middle = (vec_len /2 ) as f64 + 0.5;
        let median = (all_info_no_zero[upper_middle as usize].0 + all_info_no_zero[lower_middle as usize].0)/2;
        println!("the median distance is {:?}",median)
    }

    

}

pub fn stdev(all_info_no_zero: &Vec<(usize,usize,usize)>) {

    let mean = find_average(&all_info_no_zero);

    let mut dev = 0.0;

    for i in all_info_no_zero.iter() {

        dev += (i.0 as f64 - mean) * (i.0 as f64 - mean);
        
    }

    let variance = dev / (all_info_no_zero.len()) as f64;
    let stdev = variance.sqrt();

    println!("the standard deviation is {:?}", stdev);

}
