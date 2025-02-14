fn main() {
    let mut x = 5; 
    { // Create a scope for the mutable reference 
        let y = &mut x; 
        *y = 6; 
    } 
    { // Create another scope to handle the other mutable reference
        let z = &mut x; 
        *z = 7; 
    } 
    println!("x = {}", x); 
}