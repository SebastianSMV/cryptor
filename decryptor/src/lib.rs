pub fn decrypt(value: &str, seed: usize) -> String{
    // Base is the alphabet the user can use in their message
    let base: Vec<char> = vec![
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o',
    'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 
    'E', 'F','G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 
    'T', 'U', 'V', 'W','X', 'Y', 'Z',
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
    '!','?','-','_','*','=',' ','.',',','+','<','>','\\','/','|','\'','(',')','{','}','[',']']; 

    let mut base_encrypted = base.clone();
    let seed_sum = sum_number(seed);
    // Encrypts the base vector by swapping the position of char's in the base by using the seed
    for i in 0..seed_sum{
        for c in &base{
            let curr_num = base.iter().position(|x| x == c).unwrap();
            let pow_number: usize = wrap_number(0, seed_sum - 1, (curr_num + i) as isize);
            let num_to_get: usize = seed_sum.wrapping_pow(pow_number.try_into().unwrap()) % base.len();
            base_encrypted.swap(curr_num, wrap_number(0, base.len() - 1, num_to_get as isize));
        };
    };
    // Uses the new encyrpted base to change out the letters and change their position by using the seed
    let mut decrypted_value: String = String::new();
    for (i,c) in value.chars().enumerate(){
        let mut location_letter = base_encrypted.iter().position(|x| x == &c).unwrap();
        let seed_num = seed.to_string().chars().nth(wrap_number(0, seed.to_string().len() - 1, i as isize));
        location_letter = wrap_number(0 ,base.len() - 1, location_letter as isize - seed_num.unwrap().to_digit(10).unwrap() as isize);
        decrypted_value.push(base_encrypted[location_letter]);
    }
    decrypted_value
}
// Adds all of the numbers in a number together, e.g. 123 -> 1+2+3 = 6
fn sum_number(number: usize) -> usize{
    let mut sum: usize = 0;
    for i in number.to_string().chars(){
        let digit: usize = i.to_digit(10).unwrap() as usize;
        sum += digit;
    }
    sum
}
// Wraps the given number within the range, e.g. min: 3, max: 10, number: 12 -> 5
fn wrap_number(min: usize, max: usize, number: isize) -> usize{
    let nrange = max - min + 1;
    if number > max as isize{
        number as usize % nrange + min
    }
    else if number < min as isize{
        (nrange as isize - (number.abs() % nrange as isize)) as usize
    }else {number as usize}
}
