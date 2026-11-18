use colored::*;

struct Solution;

type I32Matrix = Vec<Vec<i32>>;
type CharMatrix = Vec<Vec<char>>;

/**
 * Implement your solution here
 *
 */
impl Solution {
    fn next(i: usize, j: usize, dir: char) -> (usize, usize) {
        match dir {
            'u' => (i - 1, j),
            'd' => (i + 1, j),
            'l' => (i, j - 1),
            'r' => (i, j + 1),
            _ => (i, j), // Caso por defecto: si es un espacio
                         // u otro carácter, no se mueve
        }
    }

    fn is_border_next(map: &CharMatrix, i: usize, j: usize, dir: char) -> bool {
        // hay que recordar que no podemos hacer indices negativos para un vec o matriz,
        // por lo que tendremos que usar un Option para determinar los bordes de nuestra matriz:
        // un None (indices ilegales) determinan el borde de la matriz
        let coords = match dir {
            'u' => {
                if i > 0 {
                    Some((i - 1, j))
                } else {
                    None
                }
            }
            'd' => Some((i + 1, j)),
            'l' => {
                if j > 0 {
                    Some((i, j - 1))
                } else {
                    None
                }
            }
            'r' => Some((i, j + 1)),
            _ => Some((i, j)),
        };

        // un Some determina que aun no tocamos el borde, pero hay que revisar si ese valor no es un
        // borde artificial (un valor que ya fue marcado)
        if let Some(crds) = coords {
            return match map.get(crds.0).and_then(|row| row.get(crds.1)) {
                Some(&mark) => mark == 'x',
                None => true,
            };
        }

        true
    }

    fn turn(matrix: &I32Matrix, map: &CharMatrix, i: usize, j: usize, curr_dir: char) -> char {
        let mut new_dir: char = ' ';

        // Generamos las coordenadas. Si se salen (son < 0), guardamos None.
        let directions = [
            ('u', if i > 0 { Some((i - 1, j)) } else { None }),
            ('d', Some((i + 1, j))),
            ('l', if j > 0 { Some((i, j - 1)) } else { None }),
            ('r', Some((i, j + 1))),
        ];

        for &(dir, coords_opt) in directions.iter() {
            // Evitamos volver por donde veníamos
            if curr_dir != dir {
                // Desempaquetamos la coordenada solo si es válida (no es None)
                if let Some(coords) = coords_opt {
                    // Verificamos si la posición existe físicamente en la matriz
                    if matrix
                        .get(coords.0)
                        .and_then(|row| row.get(coords.1))
                        .is_some()
                    {
                        // y verificamos que nuestro mapa no este marcado
                        // como visitado en dichas cordenadas
                        if map[coords.0][coords.1] == 'o' {
                            new_dir = dir;
                        }
                    }
                }
            }
        }

        new_dir
    }
    pub fn spiral_order(matrix: I32Matrix) -> Vec<i32> {
        // vector que acumulara la respuesta
        let mut ans: Vec<i32> = vec![];
        // declarar la matriz de mapeo de las casillas "visitadas"
        let (n, m) = (matrix.len(), matrix[0].len());
        let mut map: CharMatrix = vec![vec!['o'; m]; n];
        // variables para la orientacion y movimiento a traves de la matriz
        let mut dir: char = 'r';
        let (mut i, mut j) = (0_usize, 0_usize);

        // primer valor capturado y marcado
        ans.push(matrix[i][j]);
        map[i][j] = 'x';

        while ans.len() < n * m {
            // revisamos si primero hay que revirar
            if Solution::is_border_next(&map, i, j, dir) {
                dir = Solution::turn(&matrix, &map, i, j, dir);
            }
            // avanzamos en la direccion acordada
            // y vamos marcando en nuestro mapa
            (i, j) = Solution::next(i, j, dir);
            ans.push(matrix[i][j]);
            map[i][j] = 'x';
        }

        ans
    }
}

fn main() {
    // let matrix = vec![
    //     vec![1, 2, 3, 4],
    //     vec![5, 6, 7, 8],
    //     vec![9, 10, 11, 12],
    //     vec![13, 14, 15, 16],
    // ];
    let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let ans = Solution::spiral_order(matrix);
    println!("{}", format!("{:?}", ans).green().italic().underline());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spiral_matrix() {
        let cases = [
            (
                vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
                vec![1, 2, 3, 6, 9, 8, 7, 4, 5],
            ),
            (
                vec![
                    vec![1, 2, 3, 4],
                    vec![5, 6, 7, 8],
                    vec![9, 10, 11, 12],
                    vec![13, 14, 15, 16],
                ],
                vec![1, 2, 3, 4, 8, 12, 16, 15, 14, 13, 9, 5, 6, 7, 11, 10],
            ),
            (vec![vec![3], vec![2]], vec![3, 2]),
        ];

        for (input, expected) in cases {
            assert_eq!(
                Solution::spiral_order(input.clone()),
                expected,
                "{}",
                format!("{:?}", input).red().italic().underline()
            );
        }
    }
}
