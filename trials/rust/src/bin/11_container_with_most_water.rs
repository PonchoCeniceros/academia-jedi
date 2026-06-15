use colored::*;
// use katas::s;

use std::cmp::min;

struct Solution;

/**
 * Implement your solution here
 *
 */
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let len: usize = height.len();
        let mut i = 0_usize;
        let mut j = len - 1;

        println!(
            "{} ",
            format!(" i => j : {} => {}", i, j)
                .black()
                .bold()
                .on_bright_red()
        );

        loop {
            if i as i32 >= len as i32 {
                break;
            }

            if j as i32 <= 0 {
                break;
            }

                let area = ((j - i) as i32) * min(height[i], height[j]);

            println!(
                "{} ",
                format!(
                    "area = ({} - {}) * min({}, {}) => {}",
                    i, j, height[i], height[j], area
                )
                .black()
                .bold()
                .on_bright_blue()
            );

            println!(" height[i] < height[j] : {} < {} : {}", height[i], height[j], height[i] < height[j]);

            if height[i] < height[j] {
                println!("{}: {}", 1, height[i] >= height[i + 1]);
                while height[i] >= height[i + 1] {
                    println!("i: {}", i);
                    i += 1;
                }
            } else {
                println!("{}", 2);
                while height[j] >= height[j - 1] {
                    println!("j: {}", j);
                    j -= 1;
                }
            }
        }

        0
        /*
            O(n² triangular):

            area maxima = 0
            recorrer height con i:
                a partir de i voy a recorrer lo que queda de height con j:
                    calcular la base b = j - i
                    calcular la altura a = get_height(h_i,h_j)
                    obtener el area = b*a
                    guardar el area maxima


        let mut area = -1;

        for i in 0..height.len() {
            for j in (i + 1)..height.len() {
                let b = (j - i) as i32;
                let a = Solution::get_height(height[i], height[j]);
                area = if (a * b) > area { a * b } else { area }
            }
        }

        area

        */
    }
}

fn main() {
    let height = vec![1, 2, 4, 3];
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

/*
impl Solution {
    fn get_height(h_i: i32, h_j: i32) -> i32 {
        if h_i <= h_j { h_i } else { h_j }
    }

    fn get_max_h(height: &[i32]) -> usize {
        let mut aux = -1;
        let mut i = 0_usize;

        for (u, h) in height.iter().enumerate() {
            if *h > aux {
                aux = *h;
                i = u;
            }
        }

        i
    }

    fn get_min_h(height: &[i32]) -> usize {
        let mut aux = 10001;
        let mut j = 0_usize;

        for (u, h) in height.iter().enumerate() {
            if *h < aux {
                aux = *h;
                j = u;
            }
        }

        j
    }

    pub fn max_area(height: Vec<i32>) -> i32 {

        //     O(n² triangular):
        //
        //     area maxima = 0
        //     recorrer height con i:
        //         a partir de i voy a recorrer lo que queda de height con j:
        //             calcular la base b = j - i
        //             calcular la altura a = get_height(h_i,h_j)
        //             obtener el area = b*a
        //             guardar el area maxima
        //
        //
        // let mut area = -1;
        //
        // for i in 0..height.len() {
        //     for j in (i + 1)..height.len() {
        //         let b = (j - i) as i32;
        //         let a = Solution::get_height(height[i], height[j]);
        //         area = if (a * b) > area { a * b } else { area }
        //     }
        // }
        //
        // area


            // O(4n):
            //
            // i = obtener del valor maximo de height
            // para cada k en height:
            //     calcular la base b = |k - i|
            //     calcular la altura a = get_height(h_i, h_k)
            //     obtener el area = b*a
            //     guardar el area maxima
            //
            // j = obtener del valor minimo de height
            // para cada k en height:
            //     calcular la base b = |k - j|
            //     calcular la altura a = get_height(h_j, h_k)
            //     obtener el area = b*a
            //     guardar el area maxima

        let mut area = -1;
        let i = Solution::get_max_h(&height);
        let j = Solution::get_min_h(&height);

        println!(
            "{} ",
            format!(" max => h[{}] = {} ", i, height[i])
                .black()
                .bold()
                .on_bright_red()
        );

        for k in 0..height.len() {
            if k != i {
                let b = ((k as i32) - (i as i32)).abs();
                let a = Solution::get_height(height[i], height[k]);

                print!(
                    "{}",
                    format!(" b => |{} - {}| = {} ", k, i, b)
                        .black()
                        .bold()
                        .on_bright_purple()
                );

                print!(
                    "{}",
                    format!(" a => M({}, {}) = {} ", height[i], height[k], a)
                        .black()
                        .bold()
                        .on_bright_cyan()
                );

                println!(
                    "{}",
                    format!(" a*b => {}*{} = {} ", a, b, a * b)
                        .black()
                        .bold()
                        .on_bright_blue()
                );

                area = if (a * b) > area { a * b } else { area }
            }
        }

        println!(
            "{}",
            format!(" min => h[{}] = {} ", j, height[j])
                .black()
                .bold()
                .on_bright_yellow()
        );

        for k in 0..height.len() {
            if k != j {
                let b = ((k as i32) - (j as i32)).abs();
                let a = Solution::get_height(height[j], height[k]);

                print!(
                    "{}",
                    format!(" b => |{} - {}| = {} ", k, j, b)
                        .black()
                        .bold()
                        .on_bright_purple()
                );

                print!(
                    "{}",
                    format!(" a => m({}, {}) = {} ", height[j], height[k], a)
                        .black()
                        .bold()
                        .on_bright_cyan()
                );

                println!(
                    "{}",
                    format!(" a*b => {}*{} = {} ", a, b, a * b)
                        .black()
                        .bold()
                        .on_bright_blue()
                );

                area = if (a * b) > area { a * b } else { area }
            }
        }

        area
    }
}
*/
