fn main() {
    let arr: [usize; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let key = 4;
    let mut flag = false;
    let length: usize = arr.len();

    for i in 0..length {
        if key == arr[i] {
            flag = true;
        }
    }

    if flag == true {
        println!("The number exits");
    } else {
        println!("The number does not exists");
    }
}
