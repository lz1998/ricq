pub fn group_code2uin(code: i64) -> i64 {
    let mut left = code / 1000000;
    if left >= 0 && left <= 10 {
        left += 202
    } else if left >= 11 && left <= 19 {
        left += 469
    } else if left >= 20 && left <= 66 {
        left += 2080
    } else if left >= 67 && left <= 156 {
        left += 1943
    } else if left >= 157 && left <= 209 {
        left += 1990
    } else if left >= 210 && left <= 309 {
        left += 3890
    } else if left >= 310 && left <= 335 {
        left += 3490
    } else if left >= 336 && left <= 386 {
        //335 336不确定
        left += 2265
    } else if left >= 387 && left <= 499 {
        left += 3490
    }
    return left * 1000000 + code % 1000000;
}

pub fn group_uin2code(uin: i64) -> i64 {
    let mut left = uin / 1000000;
    if left >= 202 && left <= 212 {
        left -= 202
    } else if left >= 480 && left <= 488 {
        left -= 469
    } else if left >= 2100 && left <= 2146 {
        left -= 2080
    } else if left >= 2010 && left <= 2099 {
        left -= 1943
    } else if left >= 2147 && left <= 2199 {
        left -= 1990
    } else if left >= 2600 && left <= 2651 {
        left -= 2265
    } else if left >= 3800 && left <= 3989 {
        left -= 3490
    } else if left >= 4100 && left <= 4199 {
        left -= 3890
    }
    return left * 1000000 + uin % 1000000;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_code2uin() {
        let uin = group_code2uin(335783090);
        assert_eq!(uin, 3825783090);
    }
    #[test]
    fn test_group_uin2code() {
        let code = group_uin2code(3825783090);
        assert_eq!(code, 335783090);
    }
}
