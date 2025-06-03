pub fn xy_from_n(n: u8) -> (u8, u8) {
    let y: u8 = n - (n % 4) - if ( 
        if n == 0 {
            1
        } else {
            n 
        } % 4
    ) == 0 {
        4
    } else {
        0
    } / 4;

    let x: u8 = (
        if n % 4 == 0 {
            4
        } else {
            n % 4
        } * 2
    ) - 1 - if y % 2 == 0 {
        1
    } else {
        0
    };
    return (x, y);
}