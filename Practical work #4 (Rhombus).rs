#[test] {
    fn main() {
        const SIZE: usize = 5; 
        
        let mut diamond = String::new();
        
        for i in 0..SIZE {
            diamond.push_str(&" ".repeat(SIZE - i - 1)); 
            diamond.push_str(&"*".repeat(2 * i + 1));   
            diamond.push('\n');                         
        }
        
        for i in (0..SIZE - 1).rev() {
            diamond.push_str(&" ".repeat(SIZE - i - 1)); 
            diamond.push_str(&"*".repeat(2 * i + 1));   
            diamond.push('\n');                         
        }
        
        print!("{}", diamond); 
    }
}
