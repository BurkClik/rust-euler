pub fn problem_2() -> u32 {
    let mut f_value = 1;
    let mut s_value = 2;
    let mut sum = 0;

    while s_value < 4_000_000 {
        let temp = s_value;
        if s_value % 2 == 0 {
            sum += s_value;
        };
        s_value += f_value;
        f_value = temp;
    };
    sum
}
