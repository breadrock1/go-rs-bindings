use std::ffi::c_int;

pub fn add(left: i32, right: i32) -> i32 {
    left + right
}

pub fn del(left: i32, right: i32) -> i32 {
    left - right
}

#[no_mangle]
pub extern "C" fn addC(left: c_int, right: c_int) -> c_int {
    return add(left, right)
}

#[no_mangle]
pub extern "C" fn delC(left: c_int, right: c_int) -> c_int {
    return del(left, right)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_del() {
        let result = del(4, 2);
        assert_eq!(result, 2);
    }
}
