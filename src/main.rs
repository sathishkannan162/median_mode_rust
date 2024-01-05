use std::collections::HashMap;

fn main() {
    let mut vector = vec![1, 2, 4, 6, 8, 3, 3, 9, 10];
    vector.sort();
    let median = get_median(&vector);
    println!("median {median}");

    let mode = get_mode(&vector);
    
    println!("mode {mode}");
}

fn get_median(arr: &Vec<i32>) -> f64 {
    let length = arr.len();
    if length % 2 == 0 {
        let total = arr[(length / 2) - 1] + arr[length / 2];
        let result = f64::from(total) / 2.0;

        return result;
    } else {
        return f64::from(arr[(length) / 2]);
    }
}

fn get_mode(arr: &Vec<i32>) -> &i32 {
    let mut count_map: HashMap<&i32, i32> = HashMap::new();

    for ele in arr {
        let count = count_map.entry(&ele).or_insert(0);
        *count += 1;
    }
    let mut temp_value: i32 = 0;
    let mut temp_key = &arr[0];
    for (key, value) in count_map {
        if value > temp_value {
            temp_value = value;
            temp_key = key;
        }
    }
    return temp_key
}
