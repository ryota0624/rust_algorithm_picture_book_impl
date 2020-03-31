fn gcm(a: i128, b: i128) -> i128 {
    let mod_result = a % b;
    if mod_result == 0 {
        b
    } else {
        gcm(b, mod_result)
    }
}


#[test]
fn test() {
    assert_eq!(gcm(1112, 695), 139);
}
