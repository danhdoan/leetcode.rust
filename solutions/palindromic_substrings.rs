// ============================================================================

struct Solution {
}

impl Solution {
    pub fn count_odd(s: &String) -> i32 {
        let v: Vec<char> = s.chars().collect();

        let mut ans = 0;
        for i in 0..s.len() {
            let mut l = i as i32;
            let mut r = i as i32;
            while l >= 0 && r < s.len() as i32 && 
                v[l as usize] == v[r as usize] {
                ans += 1;
                l -= 1;
                r += 1;
            }
        }
        ans
    }

    pub fn count_even(s: &String) -> i32 {
        let v: Vec<char> = s.chars().collect();

        let mut ans = 0;
        for i in 0..s.len() {
            let mut l = i as i32;
            let mut r = (i+1) as i32;
            while l >= 0 && r < s.len() as i32 && 
                v[l as usize] == v[r as usize] {
                ans += 1;
                l -= 1;
                r += 1;
            }
        }
        ans
    }

    pub fn count_substrings(s: String) -> i32 {
        Solution::count_odd(&s) + Solution::count_even(&s)
    }
}

// ============================================================================

fn main() {
}

// ============================================================================

#[test]
fn test_1() {
    let s = String::from("abc");
    assert_eq!(Solution::count_odd(&s), 3);
    assert_eq!(Solution::count_even(&s), 0);
    assert_eq!(Solution::count_substrings(s), 3);
}

// ============================================================================

#[test]
fn test_2() {
    assert_eq!(Solution::count_substrings(String::from("aaa")), 6);
}

// ============================================================================
