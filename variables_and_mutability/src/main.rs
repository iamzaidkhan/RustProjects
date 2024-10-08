fn main() {
   
   let a = match random_number().checked_add(80) {
        Some(num) => num,
        None => {
            println!("Cannot Add");
            return;
        }
   };

   println!("my age is {a}")
}

fn random_number() -> u8 {
    200
}