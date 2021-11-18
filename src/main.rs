mod rule;



fn main() {
    let r = rule::Rule::parse("$.event.type eq success");
    println!("Hello, world! {:?}", r);
}