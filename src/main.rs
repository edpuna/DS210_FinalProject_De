use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
use std::collections::VecDeque;

mod sep_module;
use crate::sep_module::delete_nones;
use crate::sep_module::find_average;
use crate::sep_module::six_deg_check;
use crate::sep_module::highest_dist;
use crate::sep_module::find_median;
use crate::sep_module::stdev;


fn main() -> Result<(), Box<dyn std::error::Error>> {

    let result = read_file("M1Anonymized.csv");
    //println!("{:?}",result[result.len()-1].0+1);

    let adjacency_list = make_adjacency(result);
    
    let mut distances = vec![0;(adjacency_list.len() * (adjacency_list.len()+1))/2];
    let mut all_info: Vec<(usize,usize,usize)> = Vec::new();

    make_all_distances(&adjacency_list, &mut distances, &mut all_info);


    //println!("{:?}", distances); 
    //println!("{:?}-----------------------------------------------", all_info) ;
    //println!("{:?}-----------------------------------------------",adjacency_list);

    println!("{:?}", complete_info(&all_info));

    Ok(())

}


fn read_file(path: &str) -> Vec<(u16, u16)> {  //read_file function to create result Vector of tuples that represent the start and end nodes
    let mut result: Vec<(u16, u16)> = Vec::new();
    let file = File::open(path).expect("Could not open file");
    let buf_reader = std::io::BufReader::new(file).lines();
    for line in buf_reader {
        let line_str = line.expect("Error reading");
        let v: Vec<&str> = line_str.trim().split(',').collect();
        if v.len() == 1{    
                continue;
        }
        let x = v[0].parse::<u16>().unwrap()-1;
        let y = v[1].parse::<u16>().unwrap()-1;
        result.push((x, y));
    }
    return result;
}


fn make_adjacency(result:Vec<(u16, u16)>) -> Vec<Vec<usize>> {

    let num_nodes = result[result.len()-1].0+1;
    let mut adjacency_list : Vec<Vec<usize>> = vec![vec![];num_nodes as usize];
    for (a,b) in result.iter() {
        let a_us = usize::try_from(*a).unwrap();
        let b_us = usize::try_from(*b).unwrap();
        adjacency_list[a_us].push(b_us);
        adjacency_list[b_us].push(a_us)
    };


    return adjacency_list

}

fn make_all_distances(adjacency_list: &Vec<Vec<usize>>, distances: &mut Vec<usize>, all_info: &mut Vec<(usize,usize,usize)>) {

    let mut index = 0;
    let mut now_on = 0;
    let length = adjacency_list.len();
    for start in 0..length{
        // loop through all possible connection options
        for end in start..length{
            if let Some(x) = find_dist(&adjacency_list, start, end) {

                distances[index] = find_dist(&adjacency_list, start, end).unwrap();
                //println!("the distance between {} and {} is {:?}. that was the {} distance",start, end, distances[index],now_on);
                all_info.push((distances[index], start, end));
                index+=1;
                now_on+=1;
            }

            else {
                index+=1;
                now_on+=1;
                //println!("there is no path between {} and {}", start, end);
                all_info.push((0,start, end));
            }
        }

    }

    //println!("{:?}",all_info)

}



fn find_dist(adjacency_list: &Vec<Vec<usize>>, start: usize, end: usize) -> Option<usize> {
    let mut queue = VecDeque::new();
    let mut visits = vec![false; adjacency_list.len()]; 

    visits[start] = true;
    queue.push_back((start, 0));

    while let Some((node, distance)) = queue.pop_front() {
        if node == end {
            return Some(distance);
        }
        for &neighbor in &adjacency_list[node]{
            if !visits[neighbor] {
                visits[neighbor] = true;
                queue.push_back((neighbor, distance+1));
            }

        }
       
    }
    None

}


fn complete_info(all_info: &Vec<(usize,usize,usize)>) {


    let all_info_no_zero = delete_nones(&all_info);

    println!("the average distance is {:?}", find_average(&all_info_no_zero));
    println!("{:?}", six_deg_check(all_info));
    println!("{:?}", highest_dist(all_info));
    println!("{:?}", find_median(all_info.to_vec()));
    println!("{:?}", stdev(&all_info_no_zero))

}

#[test]
fn dist0() {
    let mut adjacency_list : Vec<Vec<usize>> =  vec![vec![],vec![1], vec![3], vec![2]];
    assert_eq!(find_dist(&adjacency_list,1,1), Some(0));
}
#[test]

fn dist1() {
    let mut adjacency_list : Vec<Vec<usize>> =  vec![vec![],vec![1], vec![3], vec![2]];
    assert_eq!(find_dist(&adjacency_list, 2,3), Some(1))

}
#[test]

fn distna() {
    let mut adjacency_list : Vec<Vec<usize>> =  vec![vec![],vec![1], vec![3], vec![2]];
    assert_eq!(find_dist(&adjacency_list, 1,2),None)

}
