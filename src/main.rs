extern crate lazy_static;
extern crate rayon;

mod matrix;
mod security_pattern;

fn main() {
    use security_pattern::SecurityPattern;
    let mut sp_n: SecurityPattern = SecurityPattern::new(3);
    println!("{}", sp_n.adj_matrix());
}
