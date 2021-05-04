///
/// Link: https://leetcode.com/problems/subrectangle-queries
///

// ============================================================================

struct SubrectangleQueries {
    rec: Vec<Vec<i32>>,
    uds: Vec<(i32, i32, i32, i32, i32)>
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SubrectangleQueries {

    fn new(rectangle: Vec<Vec<i32>>) -> Self {
        Self {
            rec: rectangle,
            uds: Vec::new(),
        }
    }

    fn update_subrectangle(&mut self, r1: i32, c1: i32, r2: i32, c2: i32, v: i32) {
        &self.uds.push((r1, c1, r2, c2, v));
    }

    fn get_value(&self, r: i32, c: i32) -> i32 {
        let mut ans = *(&self.rec[r as usize][c as usize]);
        for t in &self.uds {
            if t.0 <= r && r <= t.2 && t.1 <= c && c <= t.3 {
                ans = t.4;
            }
        }

        ans
    }
}


// ============================================================================

fn main() {
    
}

// ============================================================================
