use colored::*;
use std::cmp::min;

struct Solution;

/**
 * Implement your solution here
 *
 */
impl Solution {
    pub fn naive_max_area(height: Vec<i32>) -> i32 {
        /*
        O(n² triangular):

        area maxima = 0
        recorrer height con i:
            a partir de i voy a recorrer lo que queda de height con j:
                calcular la base b = j - i
                calcular la altura a = min(h_i, h_j)
                obtener el area = b * a
                guardar el area maxima
        */

        let mut area = -1;

        for i in 0..height.len() {
            for j in (i + 1)..height.len() {
                let b = (j - i) as i32;
                let a = min(height[i], height[j]);
                area = if (a * b) > area { a * b } else { area }
            }
        }

        area
    }

    pub fn max_area(height: Vec<i32>) -> i32 {
        /*
        O(n):

        i = 0
        j = height.last

        mientras i e j no se crucen:
            calcular la base b = j - i
            calcular la altura a = min(h_i, h_j)
            obtener el area = b * a
            guardar el area maxima

            si height_i es mas chico que height_j:
                movemos i hacia adelante
            si no:
                movemos j hacia atras
        */

        let mut area = -1;
        let mut i = 0_usize;
        let mut j = height.len() - 1;

        while i < j {
            let b = (j - i) as i32;
            let a = min(height[i], height[j]);
            area = if (a * b) > area { a * b } else { area };

            if height[i] < height[j] {
                i += 1;
            } else {
                j -= 1;
            }
        }

        area
    }
}

fn main() {
    let height = vec![1, 8, 2, 5, 4, 8, 3, 7];
    let ans = Solution::max_area(height);
    println!("{}", format!("{}", ans).black().bold().on_bright_green());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_container_with_most_water() {
        assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
        assert_eq!(Solution::max_area(vec![1, 1]), 1);
        assert_eq!(Solution::max_area(vec![1, 2, 1]), 2);
        assert_eq!(Solution::max_area(vec![1, 2, 4, 3]), 4);
    }
}
