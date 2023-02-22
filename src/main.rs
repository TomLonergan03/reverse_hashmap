mod reverse_hashmap;

fn main() {
    let mut person_and_age: reverse_hashmap::ReverseHashMap<String, u16> =
        reverse_hashmap::ReverseHashMap::new();

    person_and_age.insert("John".to_string(), 20);
    person_and_age.insert("Jane".to_string(), 54);
    person_and_age.insert("Joe".to_string(), 75);

    let a_key: u64 = rand::random();
    let value: Option<&u16> = person_and_age.get(a_key);
    if value.is_some() {
        println!("The value at key {} is {}", a_key, value.unwrap());
    } else {
        println!("There is no value at key {}", a_key);
    }
    let another_key: u64 = rand::random();
    let delete_result: Option<u16> = person_and_age.delete(another_key);
    if delete_result.is_some() {
        println!("Deleted value at key {}", another_key);
    } else {
        println!("There was no value at key {}", another_key);
    }
    let john: Option<u64> = person_and_age.find_key(20, true);
    if john.is_some() {
        println!("John is {} years old", john.unwrap());
    } else {
        println!("John is not in the hashmap");
    }
    return;
}
