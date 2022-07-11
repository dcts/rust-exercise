/**
 * BASIC SINTAX
 * ============
 *
 *
 */

// Return the sum of a and b
pub fn sum(a: u32, b: u32) -> u32 {
    return a + b;
}

// Return the multiplication of a and b
pub fn multiply(a: u32, b: u32) -> u32 {
   return a * b;
}

// Return the maximum number between a and b
// Eg: max(3, 4) = 4
pub fn max(a: u32, b: u32) -> u32 {
    if a > b {
        return a;
    } else {
        return b;
    }
}

// Return the average for the two given numbers
// Eg: average(2, 3) = 2.5
pub fn average(a: u32, b: u32) -> f32 {
    let n: f32 = 2.0;
    return (a + b) as f32 / n;
}

// Return the factorial for the given number
// The factorial of a number is the product of all its previous numbers
// Eg: factorial(4) = 1 * 2 * 3 * 4 = 24
pub fn factorial(n: u32) -> u32 {
    let mut product: u32 = 1;
    for num in 1..=n {
        product = product * num;
    }
    return product;
    // BETTER WAY
    // return (1..=n).product();
}

// Return whether the given number is a prime number
// Eg: is_prime(7) = true
// Eg: is_prime(8) = false
pub fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..n { // do not include the final number
        if n % i == 0 {
            return false;
        }
    }
    return true;
}

// Return the nth prime, assuming that the first prime number is 2
// Eg: nth_prime(1) = 2
// Eg: nth_prime(2) = 3
// Eg: nth_prime(3) = 5
pub fn nth_prime(n: u32) -> u32 {
    if n > 1000 {
        panic!("I'm gonna stop you right here... Don't overuse my memory, please!")
    }
    let mut prime_count: u32 = 0;
    let mut current_num: u32 = 2;
    let mut current_prime: u32 = 0;
    while prime_count < n {
        if is_prime(current_num) {
            prime_count += 1;
            current_prime = current_num;
        }
        current_num += 1;
    }
    return current_prime;
}

/**
 * PRIMITIVE COMPOUND TYPES
 * ========================
 *
 *
 */

/// Tuples

// Return the first
pub fn return_first_member(tuple: (u32, u32)) -> u32 {
    return tuple.0;
}

// Return the sum of all the members of the tuple
// Eg: sum((1, 2, 3)) == 6
pub fn sum_members(tuple: (u32, u32, u32)) -> u32 {
    return tuple.0 + tuple.1 + tuple.2;
}

// Build a tuple of 3 members that are all equal to the given number
// Eg: triplicate_number(1) == (1, 1, 1)
pub fn triplicate_number(n: u32) -> (u32, u32, u32) {
    let tuple: (u32, u32, u32) = (n, n, n);
    return tuple;
}

// Build a tuple of 3 members that are all equal to the given string
// Eg: triplicate_string(String::from("a")) == (String::from("a"), String::from("a"), String::from("a"))
pub fn triplicate_string(s: String) -> (String, String, String) {
    return (
        String::from(&s), 
        String::from(&s), 
        String::from(&s),
    )
}

/// Arrays

// Return an array of 5 elements, and fill it with the given number
// Eg: build_array(2) == [2, 2, 2, 2, 2]
pub fn build_array(n: u32) -> [u32; 5] {
    return [n; 5];
}

// Return the nth element of the array
// Eg: nth_element([2, 5, 3, 8, 1], 2) == 3
pub fn nth_element(array: [u32; 5], n: usize) -> u32 {
    return array[n];
}

// Return the maximum number in the given array
// Eg: max_in_array([2, 5, 3, 8, 1]) == 8
pub fn max_in_array(array: [u32; 5]) -> u32 {
    let mut max: u32 = array[0];
    for num in array.iter() {
        if num > &max {
            max = *num;
        }
    }
    return max;
}

// Returns true if the given string literal is either "foo" or "bar"
// Eg: is_foo_or_bar("foo") == true
// Eg: is_foo_or_bar("bar") == true
// Eg: is_foo_or_bar("another string") == false
pub fn is_foo_or_bar(s: &str) -> bool {
    if s == "foo" || s == "bar" {
        return true;
    }
    return false;
}

// Returns true if the given string literal contains the given character
// Eg: contains_char("foo", 'f') == true
// Eg: contains_char("foo", 's') == false
pub fn contains_char(s: &str, character: char) -> bool {
    let input_string = String::from(s);
    for c in input_string.chars() {
        if c == character {
            return true;
        }
    }
    return false;
}

// Returns true if the given string literal consists of just its first half repeteated twice
// Eg: is_first_half_repeated("fofo") == true
// Eg: is_first_half_repeated("ff") == true
// Eg: is_first_half_repeated("fof") == false
// Eg: is_first_half_repeated("fofoo") == false
// Eg: is_first_half_repeated("foo") == false
pub fn is_first_half_repeated(s: &str) -> bool {
    // check for uneven char sizes
    let len: usize = s.len();
    if len % 2 != 0 {
        return false;
    }
    // split into half
    let left: &str = &s[0..len/2];
    let right: &str = &s[len/2..len];
    return left == right;
}
