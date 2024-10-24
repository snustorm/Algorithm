// Tower of Hanoi Function
fn tower_of_hanoi(n: u32, source: &str, target: &str, auxiliary: &str) {
    if n == 1 {
        // Base case: move the last disk directly from source to target
        println!("Move disk 1 from {} to {}", source, target);
        return;
    }

    // Step 1: Move n-1 disks from source to auxiliary, using target as auxiliary
    tower_of_hanoi(n - 1, source, auxiliary, target);

    // Step 2: Move the nth disk from source to target
    println!("Move disk {} from {} to {}", n, source, target);

    // Step 3: Move n-1 disks from auxiliary to target, using source as auxiliary
    tower_of_hanoi(n - 1, auxiliary, target, source);
}

fn main() {
    let n = 3; // Number of disks
    tower_of_hanoi(n, "A", "C", "B");
}