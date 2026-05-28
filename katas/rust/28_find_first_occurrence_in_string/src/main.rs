// use colored::*;

struct Solution;

/**
 * Implementa tu solución aquí
 *
 */
impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let ans = haystack.find(&needle);

        match ans {
            Some(idx) => idx as i32,
            None => -1,
        }

        /*

        let haystack_arr = haystack.as_bytes();
        let needle_arr = needle.as_bytes();
        let mut ans = -1;

        if haystack_arr.len() < needle_arr.len() {
            return ans;
        }

        let mut i = 0;

        for j in 0..haystack_arr.len() {
            let x = needle_arr[i] as char;
            let y = haystack_arr[j] as char;

            println!(
                "{} {} | j={} | i={}",
                format!(" {} ", (needle_arr[i] as char))
                    .black()
                    .bold()
                    .on_bright_red(),
                format!(" {} ", (haystack_arr[j] as char))
                    .black()
                    .bold()
                    .on_bright_blue(),
                format!("{}", j).blue().bold(),
                format!("{}", i).red().bold(),
            );

            if x == y {
                i += 1;
                ans = (j as i32) - (needle_arr.len() as i32) + 1;
                if i == needle_arr.len() {
                    return ans;
                }
            } else {
                i = 0;
                ans = -1;
            }
        }
        ans
        */
    }
}

/**
 * Pruebas unitarias
 *
 */
fn main() {
    let ans = Solution::str_str(String::from("mississippi"), String::from("issip"));
    println!("{}", ans);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_first_occurrence_in_string() {
        assert_eq!(
            Solution::str_str(String::from("sadbutsad"), String::from("sad")),
            0
        );
        assert_eq!(
            Solution::str_str(String::from("leetcode"), String::from("leeto")),
            -1
        );
        assert_eq!(
            Solution::str_str(String::from("sacbutsadbut"), String::from("sad")),
            6
        );
    }
}
