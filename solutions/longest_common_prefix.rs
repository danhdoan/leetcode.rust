///
/// Link: https://leetcode.com/problems/longest-common-prefix/
///

// ============================================================================

struct Solution {
}

impl Solution {
    fn get_common_prefix(u: &String, v: &String) -> String {
        let u_v: Vec<char> = u.chars().collect();
        let v_v: Vec<char> = v.chars().collect();

        let len = if u_v.len() < v_v.len() {u_v.len()} else {v_v.len()};
        let mut ans: Vec<char> = Vec::new();
        for i in 0..len {
            if u_v[i] == v_v[i] {
                ans.push(u_v[i]);
            }
            else {
                break;
            }
        }

        ans.into_iter().collect()
    }

    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut ans: String = strs[0].clone();

        for i in 1..strs.len() {
            ans = Solution::get_common_prefix(&ans, &strs[i]);
        }

        ans
    }
}

// ============================================================================

fn main() {
}

// ============================================================================

#[test]
fn test_1() {
    let v = vec![String::from("flower"),
        String::from("flow"),
        String::from("flight")];
    let ans = String::from("fl");
    assert_eq!(Solution::longest_common_prefix(v), ans);
}

// ============================================================================

#[test]
fn test_2() {
    let v = vec![String::from("dog"),
        String::from("racecar"),
        String::from("car")];
    let ans = String::from("");
    assert_eq!(Solution::longest_common_prefix(v), ans);
}

// ============================================================================
