extern crate semver;

use semver::Version;

use std::sync::{Arc, Mutex};

fn main() {
	
	// Added a semvar library which parses version numbers 
	// and compares them according ti the semver specification.
	assert!(Version::parse("1.2.3") == Ok(Version {
		major: 1u,
		minor: 2u,
		patch: 3u,
		pre: vec!(),
		build: vec!(),
	}));

	println!("Version compared successfully!");
	
	// Assign mutable variable array
	// add item with "push"
	// clone the array value to "x"
	let mut v = vec![];
	v.push("Hello");
	let x = v[0].clone();
	v.push("world");

	println!("{}", x);
	
	// Spawn seperate processes "proc()" with their own calculation routines
	let mut i = 0i;
	for _ in range(0u, 10u){
		spawn(proc(){
			println!("Hello, world! {}", i);
		});
		i = i +1;
	}
	
	// Move numbers array to an enclosed environment by using
	// ARC = atomatically refeerence counted:
	// Keeps a track of the number of references to something, and will not 
	// free the associated resource until the count is zero    
	// MUTEX:
	// A MUTEX will synchronize our accesses, so that we can ensure 
	// our mutation doesn't cause a data race.
	let numbers = Arc::new(Mutex::new(vec![1i, 2i, 3i]));
	for i in range(0u, 3u){
		let number = numbers.clone();
	
		spawn(proc(){
			let mut array = number.lock();
			(*array)[i] += 1;
			println!("numbers[{}] is {}", i, (*array)[i]);
		});
	}
	
	// An if statement as an expression when no semicolon is used within curly braces
	let x = 5i; 
	let y :int = if x == 5i { 10i }else{ 15i };
	println!("{}", y);
	
	// (fig 1) Funtion expression return a value when now semicolon is used
	println!("{}", foo(10));

	// (fig 2) Tuples in functions
	// Allows for multiple values to be returned from a function
	let (x, y) = next_two(5i);
	println!("x, y = {}, {}", x, y);
	
	// (fig 3) Set struct 
	let mut point = PointInSpace { x: 0i, y: 0i };
	point.x = 5;
	println!("The origin is at ({}, {})", point.x, point.y);

}

// Fig 1: Function returns a value
fn foo(x: int) -> int {
	if x < 5 { return x; }
	
	x +1
}

// Fig 2: Function returns multiple values
fn next_two(x: int) -> (int, int) {
	(x + 1i, x + 2i)
}

// Fig 3: Define struct (object)
struct PointInSpace {
	x: int,
	y: int,
}


