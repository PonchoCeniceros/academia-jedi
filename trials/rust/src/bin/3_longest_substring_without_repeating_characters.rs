use colored::*;
use katas::s;
use std::cmp::max;
use std::collections::{HashMap, hash_map::Entry::Vacant};

struct Solution;

/**
 * Implement your solution here
 *
 */
impl Solution {
    fn get_max_len(arr: &[char]) -> i32 {
        // indice para recorrer el arreglo de chars
        let mut i = 0_usize;
        // calcular la longitud de los substrs y el maximo de ellos
        let (mut l, mut max_l) = (0, 0);

        // map para acumular los chars de la substring consultada
        // junto a sus pocisiones ne el array
        let mut set: HashMap<char, i32> = HashMap::new();

        loop {
            // condicion de finalizacion
            if i >= arr.len() {
                // obtener la longitud maxima
                max_l = max(max_l, l);
                break;
            }
            //
            // el patron de verificacion/insercion en un map:
            //
            // if !map.contains_key(&key):
            //     map.insert(key, value)
            //
            // puede ser optimizado con Vacant u Occupied
            //
            if let Vacant(e) = set.entry(arr[i]) {
                e.insert(i as i32);
                l += 1; // acumular la longitud de la substr
                i += 1; // mover el indice a traves de la substr
            } else {
                // obtener la longitud maxima
                max_l = max(max_l, l);
                // obtener la posicion en la substr actual
                // del char que se encuentra repetido
                if let Some(&r) = set.get(&arr[i]) {
                    // reposiciono el indice al siguiente char de la char repetida,
                    // el cual es el inicio de la siguiente substr
                    i = r as usize + 1;
                }
                // limpio el map y la longitud para
                // la evaluacion de la siguiente substr
                set.clear();
                l = 0;
            }
        }

        max_l
    }

    pub fn length_of_longest_substring(s: String) -> i32 {
        // arreglo de chars en lugar de String
        let arr: Vec<char> = s.chars().collect();
        // limites minimos de longitud del string
        if arr.len() <= 1 {
            return arr.len() as i32;
        }
        // calcular la longitud (antes le daba reverse al array pero ya no es necesario)
        Solution::get_max_len(&arr)
    }
}

fn main() {
    let s = s!("dvdf");
    let ans = Solution::length_of_longest_substring(s);
    println!("{}", format!("{}", ans).black().bold().on_bright_green());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_of_longest_substring() {
        let cases = [
            ("abcabcbb", 3),
            ("bbbbb", 1),
            ("pwwkew", 3),
            ("aab", 2),
            ("dvdf", 3),
            ("asjrgapa", 6),
            (" ", 1),
            ("busvutpwmu", 7),
        ];

        for (input, expected) in cases {
            assert_eq!(
                Solution::length_of_longest_substring(s!(input)),
                expected,
                "{}",
                format!("{:?}", input).red().italic().underline()
            );
        }
    }
}
