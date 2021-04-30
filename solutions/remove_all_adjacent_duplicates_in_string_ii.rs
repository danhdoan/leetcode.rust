///
/// Link: https://leetcode.com/problems/remove-all-adjacent-duplicates-in-string-ii
///

// ============================================================================

struct Solution {
}

impl Solution {
    pub fn remove_duplicates(s: String, k: i32) -> String {
        let mut vec: Vec<(char, usize)> = Vec::new();

        for c in s.chars() {
            let vec_t = match vec.last_mut() {
                Some(v) => v,
                _ => {
                    vec.push((c, 1));
                    continue;
                }
            };

            if vec_t.0 == c {
                vec_t.1 += 1;
            }
            else {
                vec.push((c, 1));
                continue;
            }
            if vec_t.1 == k as usize {
                vec.pop();
            }
        }

        vec.into_iter().flat_map(|x| vec![x.0; x.1]).collect()
    }
}

// ============================================================================

fn main() {
}

// ============================================================================

#[test]
fn test_1() {
    let s = String::from("abcd");
    let k = 2;
    let ans = String::from("abcd");
    assert_eq!(Solution::remove_duplicates(s, k), ans);
}

// ============================================================================

#[test]
fn test_2() {
    let s = String::from("deeedbbcccbdaa");
    let k = 3;
    let ans = String::from("aa");
    assert_eq!(Solution::remove_duplicates(s, k), ans);
}

// ============================================================================

#[test]
fn test_3() {
    let s = String::from("pbbcggttciiippooaais");
    let k = 2;
    let ans = String::from("ps");
    assert_eq!(Solution::remove_duplicates(s, k), ans);
}

// ============================================================================
