#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

fn main() {
    let mut alice = Person {
        name: String::from("Alice"),
        age: 30,
    };
    let mut bob = Person {
        name: String::from("Bob"),
        age: 20,
    };
    println!("Alice: {:?}, Bob: {:?}", alice, bob);

    birthday_immutable(&mut alice);
    println!("Alice: {:?}, Bob: {:?}", alice, bob);

    birthday_mutable(&mut alice, &mut bob);
    println!("Alice: {:?}, Bob: {:?}", alice, bob);

    // Dereference

    let mut x = 5;
    double(&mut x);
    println!("{}", x);

    // Iterators

    let mut nums = vec![1, 2, 3, 4, 5];
    for i in 1..3 {
        for j in &mut nums {
            let _: &mut u32 = j;
            println!("{}, {}", i, j);
            *j *= 2;
        }
    }

    // IntoIterator
    into_iterator_example();
}

fn birthday_immutable(person: &mut Person) {
    person.age += 1
}

fn birthday_mutable<'a>(mut person: &'a mut Person, replacement: &'a mut Person) {
    person = replacement;
    person.age += 1
}

fn double(x: &mut u32) {
    *x *= 2;
}

struct InfiniteIterator;

impl Iterator for InfiniteIterator {
    type Item = ();

    fn next(&mut self) -> Option<()> {
        Some(())
    }
}

struct InfiniteUnit;

impl IntoIterator for InfiniteUnit {
    type Item = ();
    type IntoIter = InfiniteIterator;

    fn into_iter(self) -> Self::IntoIter {
        InfiniteIterator
    }
}

fn into_iterator_example() {
    let mut count = 0;
    for _ in InfiniteUnit {
        count += 1;
        println!("count = {}", count);
        if count >= 5 {
            break;
        }
    }
}
