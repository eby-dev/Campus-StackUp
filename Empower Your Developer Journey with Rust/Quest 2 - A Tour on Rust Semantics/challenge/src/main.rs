fn main() {
    let mut even_numbers: Vec<i32> = Vec::new();
    let limit = 300;

    for num in (1..=limit).rev() {
        if num % 2 == 0 {
            println!("{}", num); 
            even_numbers.push(num);
        }
    }

    println!("{:?}", even_numbers);
}
