## 13. Roman to Integer

```rust
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let arr = s.as_bytes();
        let mut ans = 0;

        for idx in 0..arr.len() {
            // Obtenemos el valor del símbolo actual
            let current_val = Self::get_value(arr[idx]);

            // Si hay un siguiente símbolo y es mayor que el actual, restamos el actual
            if idx + 1 < arr.len() && current_val < Self::get_value(arr[idx + 1]) {
                ans -= current_val;
            } else {
                // De lo contrario, lo sumamos
                ans += current_val;
            }
        }

        ans
    }

    // Una función auxiliar que traduce el byte (u8) directamente a número.
    // Esto se compila directamente en instrucciones ultra rápidas en la CPU.
    inline fn get_value(roman_char: u8) -> i32 {
        match roman_char {
            b'I' => 1,
            b'V' => 5,
            b'X' => 10,
            b'L' => 50,
            b'C' => 100,
            b'D' => 500,
            b'M' => 1000,
            _ => 0,
        }
    }
}
```
