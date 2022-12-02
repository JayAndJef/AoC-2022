use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let list_of_inputs = input.split("\n\n");
    let mut cal_total_list = Vec::new();
    for cal_list in list_of_inputs {
        let mut total: i32 = 0;
        for cal in cal_list.split("\n") {
            total = total + cal.parse::<i32>().unwrap();
        }
        cal_total_list.push(total);
    }

    let mut best_3 = 0;

    for _ in 0..3 {
        let max = cal_total_list.iter().max().unwrap().clone();
        best_3  = best_3 + max;
        cal_total_list.retain(|&x| x != max);
    }

    println!("{}", best_3);
}