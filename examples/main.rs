use genere::Generator;

fn main() {
    let json = r#"
{
   "hero": ["John[m]", "Joan[f]", "Jon[n]"],
   "job": ["sorci·er·ère[hero]"],
   "main": ["{hero}. Il/Elle[hero] est un·e[hero] {job}."]
}"#;
        
    let mut gen = Generator::new();
    gen.add_json(json).unwrap();
    println!("{}", gen.instantiate("main").unwrap());
}
