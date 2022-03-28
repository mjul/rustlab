use std::env;

#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    // we can consume the iterator with a function call like this
    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);
}

#[test]
fn iterator_generation() {
    let v1: Vec<i32> = vec![1, 2, 3];
    // this is lazy, so it does nothing:
    // v1.iter().map(|x| x + 1);
    // we can consume it like so
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

// we can filter (select matches) using the filter function
fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
}

// We can create our own iterators

struct Counter {
    count: u32,
}
impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

#[test]
fn counter_tests() {
    let mut counter = Counter::new();
    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);

    let counter = Counter::new();
    let actual: Vec<_> = counter.collect();
    assert_eq!(vec![1, 2, 3, 4, 5], actual);
}

// the standard library has a bunch of iterator-related traits
// these are pretty vanilla
#[test]
fn using_other_iterator_trait_methods() {
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    assert_eq!(18, sum);
}

// From chapter 12
pub struct ConfigWithClone {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}
impl ConfigWithClone {
    pub fn new(args: &[String]) -> Result<ConfigWithClone, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        // we want to avoid these .clone calls:
        let query = args[1].clone();
        let filename = args[2].clone();

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(ConfigWithClone {
            query,
            filename,
            case_sensitive,
        })
    }
}

// Let's avoid the cloning and use iterators over the args
#[derive(Debug, PartialEq)]
pub struct ConfigWithIterators {
    pub query: String,
    pub filename: String,
    // simplified, no env
}
impl ConfigWithIterators
{
    // use the Args iterator
    pub fn new<T>(mut args: T) -> Result<ConfigWithIterators, &'static str> 
    where T: Iterator<Item=String>
    {
        let (c, q, f) = (args.next(), args.next(), args.next());
        match (c, q, f) {
            (_, Some(query), Some(filename)) => Ok(ConfigWithIterators { query, filename }),
            (_, _, None) => Err("not enough arguments"),
            _ => Err("this cannot happen."),
        }
    }
}

struct FakeArgs {
    count: u32,
}
impl FakeArgs {
    fn new() -> FakeArgs {
        FakeArgs { count: 0 }
    }
}
impl Iterator for FakeArgs {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        match self.count {
            1 => Some(String::from("minigrep")),
            2 => Some(String::from("query")),
            3 => Some(String::from("filename.txt")),
            _ => None,
        }
    }
}

#[test]
fn fake_args() {
    let mut a = FakeArgs::new();
    assert_eq!(String::from("minigrep"), a.next().unwrap());
    assert_eq!(Some(String::from("query")), a.next());

    for x in FakeArgs::new() {
        println!("FakeArgs: {:?}", x);
    }
}

#[test]
fn config_with_iterators() {
    let args = FakeArgs::new();

    let config = ConfigWithIterators::new(args).expect("argument parse error");
    let expected = ConfigWithIterators {
        query: String::from("query"),
        filename: String::from("filename.txt"),
    };
    assert_eq!(expected, config);
}
