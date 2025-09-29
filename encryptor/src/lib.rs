use colored::Colorize;

pub fn encrypt(value: &str, seed: usize) -> String{
    let base: Vec<char> = vec![
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o',
    'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 
    'E', 'F','G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 
    'T', 'U', 'V', 'W','X', 'Y', 'Z',
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
    '!','?','-','_','*','=',' ','.',',','+','<','>','\\','/','|','\'','(',')','{','}','[',']']; 

    for c in seed.to_string().chars(){
        if c == '0'{
            println!("{}","Warning, 0 detected. Using too many 0's could cause unwanted patterns that expose the original input.".yellow());
        }
    }
    if seed.to_string().len() < 5{
        println!("{}","Critical, extremely short seed detected. Please input a longer string to make the encryption process more secure.".red());
    }

    let mut base_encrypted = base.clone();
    let seed_sum = sum_number(seed);
    for i in 0..seed_sum{
        for c in &base{
            let curr_num = base.iter().position(|x| x == c).unwrap();
            let pow_number: usize = wrap_number(0, seed_sum - 1, curr_num + i);
            let num_to_get: usize = seed_sum.wrapping_pow(pow_number.try_into().unwrap());
            base_encrypted.swap(curr_num, wrap_number(0, base.len() - 1, num_to_get));
        };
    };
    let mut encrypted_value: String = String::new();
    for (i,c) in value.chars().enumerate(){
        let mut location_letter = base_encrypted.iter().position(|x| x == &c).unwrap();
        let seed_num = seed.to_string().chars().nth(wrap_number(0, seed.to_string().len() - 1, i));
        location_letter = wrap_number(0, base.len() - 1, location_letter + seed_num.unwrap().to_digit(10).unwrap() as usize);
        encrypted_value.push(base_encrypted[location_letter]);
    }
    encrypted_value
}
fn sum_number(number: usize) -> usize{
    let mut sum: usize = 0;
    for i in number.to_string().chars(){
        let digit: usize = i.to_digit(10).unwrap() as usize;
        sum += digit;
    }
    sum
}
fn wrap_number(min: usize, max: usize, number: usize) -> usize{
    if number > max{
        let nrange = max - min + 1;
        number % nrange + min
    }
    else {number}
}
