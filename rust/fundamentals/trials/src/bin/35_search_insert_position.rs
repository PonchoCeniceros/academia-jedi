use colored::*;

struct Solution;

/**
 * Implement your solution here
 */
impl Solution {
    //
    // BUSQUEDA BINARIA
    //
    // Condición de parada: Mantenemos el bucle mientras el rango sea válido.
    // Cuando `left > right`, significa que el elemento no está en el arreglo.
    // while left <= right {
    //
    //     Cálculo seguro del punto medio usando enteros.
    //     ** Evita desbordamiento (overflow) de memoria y aplica `floor` automático.
    //     mid = left + (right - left) / 2
    //
    //     Caso ideal: Encontramos el valor exacto en el índice mid.
    //     if target == nums[mid]
    //         return mid
    //
    //     Si el objetivo es menor, descartamos
    //     la mitad derecha incluido mid.
    //     if target < nums[mid]
    //         right = mid - 1
    //
    //     Si el objetivo es mayor, descartamos
    //     la mitad izquierda incluido mid.
    //     if target > nums[mid]
    //         left = mid + 1
    // }
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        if nums.len() == 1 {
            return if target <= nums[0] { 0 } else { 1 };
        }

        let mut left: i32 = 0;
        let mut right = nums.len() as i32 - 1;

        while left <= right {
            let mid = left + (right - left) / 2;
            let idx = mid as usize;

            if target == nums[idx] {
                return mid;
            }

            if target < nums[idx] {
                right = mid - 1;
            }

            if target > nums[idx] {
                left = mid + 1;
            }
        }

        left
    }
}

fn main() {
    let ans = Solution::search_insert(vec![8, 11, 14, 18, 22], 0);
    println!("{}", format!("{}", ans).green().italic().underline());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_insert_position() {
        let cases = [
            ((vec![1, 3], 2), 1),
            ((vec![1], 0), 0),
            ((vec![8, 11, 14, 18, 22], 0), 0),
            ((vec![8, 11, 14, 18, 22], 23), 5),
            ((vec![1, 3, 5, 6], 5), 2),
            ((vec![1, 3, 5, 6], 2), 1),
            ((vec![1, 3, 5, 6], 7), 4),
        ];

        for (input, expected) in cases {
            assert_eq!(
                Solution::search_insert(input.0.clone(), input.1),
                expected,
                "{}",
                format!("{:?}", input).red().italic().underline()
            );
        }
    }
}
