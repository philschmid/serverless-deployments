use serverless_deploy::deploy;

fn main() {
    println!("Hello, world!");
    let x = deploy(1,2);
    println!("{}",x)
}
