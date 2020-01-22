use std::io;

fn main() {

    loop {
        println!("Please insert your temperature in Celcius");

        let mut temp = String::new();

        io::stdin().read_line(&mut temp)
            .expect("Failed to read line!");

        let temp: i32 = match temp.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("This is your temp: {}", temp);

        let result = convert_temp(temp);
        
        println!("This is your converted temperature: {}!", result);

    }
}

fn convert_temp(temp: i32) -> i32 {
     (temp * 9/5) + 32
}
