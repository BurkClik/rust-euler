pub fn problem_3() -> u64 {
    // What is the largest prime of the number 600851475143?

    let mut input = 600851475143;
    let mut div = 0;
    let mut counter = 2;
    while div < input {
        if input % counter == 0 {
            input = input / counter;
            div = counter;
        } else {
            counter += 1;
        }
    };
    div
}
