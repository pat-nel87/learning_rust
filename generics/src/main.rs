pub mod trait_tut;

fn main() {
   let number_list = vec![34, 50, 25, 100, 65];
   
   let result = largest_generic(&number_list);
   println!("The largest number is {}", result);

   let char_list = vec!['n', 'm', 'o', 'y', 'z'];
   let result = largest_generic(&char_list);
   println!("The largest char is {}", result);
   
   let result = smallest_generic(&char_list);
   println!("Smallest Char: {}", result);

   let integer_point = Point {x: 5, y: 10};
   let float_point = Point {x: 1.0, y: 4.0};
   let point_two = PointTwoTypes {x: 7.5, y: 2};

   println!("Integer Point X: {:?} \n Float Point X: {:?}", integer_point.x, float_point.x);
   println!("Point Struct with two different data types \n X: {:?} \n Y: {:?}", point_two.x, point_two.y);

   TraitExample();
}

fn largest_number(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest 
}

// We specify the std::cmp::PartialOrd to say we only want to work with types
// that can be compared
// this is referred to as using "trait bounds" to specify a generic type can be any type
// that has a certain behavior
fn largest_generic<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }       
    }
    largest
}

fn smallest_generic<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut smallest = &list[0];

    for item in list {
        if item < smallest {
            smallest = item;
        }
    }
    smallest
}

// Structs can also be defined with generic type parameters
// here the values must be the same type when instantiating
struct Point<T> {
    x: T,
    y: T, 
}

// here a struct can take different generic types
struct PointTwoTypes<T, U> {
    x: T,
    y: U,
}

// we can specify constraints by type for methods implemented on generic functions / structs 
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// However because generic type parameters in a struct definition aren't always the same as those you use
// in that same struct's method signatures, We may want to be more explicit in our struct method
struct PointXY<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> PointXY<X1, Y1> {
    fn mixup<X2, Y2>(self, other: PointXY<X2, Y2>) -> PointXY<X1, Y2> {
        PointXY {
            x: self.x,
            y: other.y,
        }
    }
}


fn TraitExample() {
    use trait_tut::{Summary, Tweet};

    let tweet = Tweet {
        username: String::from("random_user_name"),
        content: String::from(
            "Random content stuff!",
        ),
        reply: false,
        retweet: false,
    };
    println!("Tweet is: {}", tweet.summarize());
}
