use std::collections::HashMap;

// --- Structs and Enums for the exercise ---

struct User {
    id: u32,
    name: String,
    status: UserStatus,
    roles: Vec<String>,
}

enum UserStatus {
    Active,
    Inactive,
    Suspended(String), // Reason for suspension
}

enum Notification {
    Welcome(String),               // User's name
    Message { from: String, content: String },
    Alert(u32),                    // Error code
    Event(String, u32, bool),      // Event name, timestamp, is_critical
    None,
}

// --- Exercise Problems ---

fn main() {
    println!("--- Exercise 1: Literal and Wildcard Patterns ---");
    exercise_1();
    println!("\n--- Exercise 2: Multiple Patterns (OR) and Ranges ---");
    exercise_2();
    println!("\n--- Exercise 3: Destructuring Structs and Ignoring Fields ---");
    exercise_3();
    println!("\n--- Exercise 4: Destructuring Enums ---");
    exercise_4();
    println!("\n--- Exercise 5: Destructuring Tuples and `if let` ---");
    exercise_5();
    println!("\n--- Exercise 6: References and `ref mut` ---");
    exercise_6();
    println!("\n--- Exercise 7: `@` Bindings (at-patterns) ---");
    exercise_7();
    println!("\n--- Exercise 8: Match Guards ---");
    exercise_8();
}

// --- Problem 1: Literal and Wildcard Patterns ---
// Match the 'color' variable and print its description.
// Use literal patterns for specific colors and a wildcard for anything else.
fn exercise_1() {
    let color = "blue"; // Try changing this to "red", "green", "yellow"

    match color {
        // YOUR CODE HERE
        "red" => println!("This is red color"),
        "blue" => println!("This is blue color"),
        _ => println!("This is unknown color")
    }

    let number = 5; // Try changing this to 1, 2, 3, 4, 5

    match number {
        // YOUR CODE HERE (Match 1, 2, and then use _ for anything else)
        1 => println!("Number 1"),
        2 => println!("Number 2"),
        _ => println!("Any other number")
    }
}

// --- Problem 2: Multiple Patterns (OR) and Ranges ---
// Match the 'score' and print the grade.
// Match 'char_code' and print if it's a digit or a lowercase vowel.
fn exercise_2() {
    let score = 85; // Try changing this to 95, 72, 60, 45, 105

    match score {
        // YOUR CODE HERE
        // 90-100: "A"
        // 80-89: "B"
        // 70-79: "C"
        // 60-69: "D"
        // 0-59: "F"
        // Anything else: "Invalid Score"
        90..=100 => println!("A"),
        80..89 => println!("B"),
        70..79 => println!("C"),
        60..69 => println!("D"),
        0..59 => println!("F"),
        _ => println!("Illegal")
    }

    let char_code = '7'; // Try changing this to 'a', 'x', 'E', '3'

    match char_code {
        // YOUR CODE HERE
        // Match '0' through '9' OR 'a', 'e', 'i', 'o', 'u'
        '0'..'9' | 'a' |'e' | 'i' | 'o' | 'u' => println!("Case A"),
        _ => println!("Unknown case")
    }
}

// --- Problem 3: Destructuring Structs and Ignoring Fields ---
// Destructure the `User` struct and print specific information.
fn exercise_3() {
    let user = User {
        id: 101,
        name: String::from("Alice"),
        status: UserStatus::Active,
        roles: vec![String::from("admin"), String::from("editor")],
    };

    // Part A: Print the user's name and ID. Ignore status and roles.
    // Use shorthand for name, explicit for id.
    // YOUR CODE HERE (use a let statement with destructuring)

    // Part B: Create a new User struct and match it.
    // - If id is 200, print "Found user 200!"
    // - If id is anything else, print "User ID: [id]" and ignore name, status, and roles.
    let user2 = User {
        id: 200,
        name: String::from("Bob"),
        status: UserStatus::Inactive,
        roles: vec![],
    };

    match user2 {
        // YOUR CODE HERE
        User {id: 200, ..} => println!("Found user 200!"),
        User {id, ..} => println!("User ID: {}", id),

    }
}

// --- Problem 4: Destructuring Enums ---
// Handle different `Notification` variants and print appropriate messages.
fn exercise_4() {
    let notification1 = Notification::Welcome(String::from("Charlie"));
    let notification2 = Notification::Message {
        from: String::from("Admin"),
        content: String::from("System update."),
    };
    let notification3 = Notification::Alert(404);
    let notification4 = Notification::Event(String::from("Meeting"), 1678886400, true);
    let notification5 = Notification::None;

    let notifications = vec![notification1, notification2, notification3, notification4, notification5];

    for notif in notifications {
        match notif {
            // YOUR CODE HERE
            // Welcome: "Welcome, [name]!"
            // Message: "New message from [from]: '[content]'"
            // Alert: "Alert: Error Code [code]"
            // Event: "New Event: [name] (Critical: [is_critical])" (ignore timestamp)
            // None: "No new notifications."
            Notification::Message {from, content} => println!("New message from {}: '{}", from, content),
            Notification::Welcome(n) => println!("Welcome, {}",n),
            Notification::Alert(c) => println!("Alert: Error Code {}", c),
            Notification::Event(event, is_critical, ..) => println!("New Event: {}, (Critical: {})", event, is_critical),
            Notification::None => println!("No new notifications")
        }
    }
}

// --- Problem 5: Destructuring Tuples and `if let` ---
// Use `if let` to specifically handle a tuple value.
fn exercise_5() {
    let coordinates = (10, 20, 30); // Try (0, 0, 0), (5, 0, 10), (0, 15, 0)

    // Use `if let` to check if the coordinates represent the origin (0, 0, 0)
    // and print "At the origin!" if true.
    // YOUR CODE HERE
    if let (0, 0, 0) = coordinates {
        println!("At the origin!")
    }


    // Use `if let` to check if the first element of `maybe_pair` is 10
    // and bind the second element to `val_b`. Print "First is 10, second is [val_b]"
    let maybe_pair: Option<(i32, i32)> = Some((10, 42)); // Try None, Some((5, 10))

    // YOUR CODE HERE
    if let Some((10, b)) = maybe_pair {
        println!("first is 10, second is {}", b)
    }
}

// --- Problem 6: References and `ref mut` ---
// Modify a value within an Option using `ref mut`.
fn exercise_6() {
    let mut data: Option<Vec<i32>> = Some(vec![1, 2, 3]);

    match data {
        // YOUR CODE HERE
        // If it's Some, use `ref mut` to get a mutable reference to the Vec.
        // Then, push a new element (e.g., 4) into the Vec.
        // Print the modified Vec.
        Some(ref mut data) => data.push(4),
        None => println!("No data")
    }
    println!("Data after match: {:?}", data); // Should show Some([1, 2, 3, 4])


    let my_string = String::from("Immutable String");
    match &my_string {
        // YOUR CODE HERE (Match a reference to the string)
        str  => println!("Its immutable string {}",str)
    }
}

// --- Problem 7: `@` Bindings (at-patterns) ---
// Use `@` to bind a value and check it against a range simultaneously.
fn exercise_7() {
    let age = 15; // Try 8, 25, 65

    match age {
        // YOUR CODE HERE
        // Match ages 13-19 (teenagers) and bind the age to a variable `teen_age`
        // Print "You are a teenager: [teen_age]"
        // Match ages 20-59 (adults) and bind to `adult_age`
        // Print "You are an adult: [adult_age]"
        // Otherwise, print "Other age: [age]"
        teen_age @ 13..19 => println!("Teen age {}",teen_age),
        adult_age @ 20..59 => println!("Adult age {}", adult_age),
        age => println!("Age {}", age)
    }

    let code = 12345; // Try 10001, 9999, 54321

    match code {
        // YOUR CODE HERE
        // Match 5-digit codes that start with 1 (e.g., 1xxxx)
        // Bind the matched code to `starting_with_one`
        // Print "Code starts with 1: [starting_with_one]"
        // Otherwise, print "Other code: [code]"
        // HINT: Use a range for 5-digit numbers starting with 1
        starting_with_one @ 10000..19999 => println!("Starting with one {}", starting_with_one),
        other_code => println!("Other code {}", other_code)
    }
}

// --- Problem 8: Match Guards ---
// Use `if` guards to add conditions to match arms.
fn exercise_8() {
    let balance = 1500;
    let transaction_amount = 600;

    match (balance, transaction_amount) {
        // YOUR CODE HERE
        // If transaction_amount is 0: "Transaction amount cannot be zero."
        // If balance is greater than or equal to transaction_amount: "Transaction successful. New balance: [new_balance]"
        // (Calculate new_balance inside the arm)
        // Otherwise: "Insufficient funds. Balance: [balance], Attempted: [amount]"
        (balance, transaction_amount) if balance >= transaction_amount => println!("Transaction successful. New balance: {}", balance - transaction_amount),
            (_, _) => println!("Insufficient funds")
    }

    let user_data = Some((String::from("Eve"), 30)); // Try None, Some((String::from("Frank"), 10))

    match user_data {
        // YOUR CODE HERE
        // If Some and age is less than 18: "User [name] is a minor."
        // If Some and age is 18 or greater: "User [name] is an adult."
        // If None: "No user data."
        Some((name, age)) if age < 18 => println!("User {} is a minor", name),
        Some((name, age)) if age > 18 => println!("User {} is a adult", name),
        _ => println!("No user data")
    }
}
