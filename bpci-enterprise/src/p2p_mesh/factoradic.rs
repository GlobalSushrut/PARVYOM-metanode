//! Factoradic (Lehmer Code) Encoding
//! 
//! Mathematical foundation for Factorial-Wave Mesh addressing.
//! 
//! # Purpose
//! 
//! Factoradic encoding provides a bijection between:
//! - Integers [0, n!-1] ↔ Permutations of n elements
//! 
//! This enables compact wave addressing in the factorial wave space.
//! 
//! # Example
//! 
//! For n=4, permutation [2,0,3,1] encodes to factoradic [2,0,1,0]:
//! - Position 0: 2 elements smaller come after (2)
//! - Position 1: 0 elements smaller come after (0)
//! - Position 2: 1 element smaller comes after (1)
//! - Position 3: 0 elements smaller come after (0)
//! 
//! Integer value: 2×3! + 0×2! + 1×1! + 0×0! = 12 + 0 + 1 + 0 = 13

/// Permutation representation
pub type Permutation = Vec<usize>;

/// Factoradic representation
pub type Factoradic = Vec<usize>;

/// Encode a permutation to factoradic (Lehmer code)
pub fn permutation_to_factoradic(perm: &[usize]) -> Factoradic {
    let n = perm.len();
    let mut factoradic = vec![0; n];
    
    for i in 0..n {
        let mut count = 0;
        for j in (i + 1)..n {
            if perm[j] < perm[i] {
                count += 1;
            }
        }
        factoradic[i] = count;
    }
    
    factoradic
}

/// Decode factoradic to permutation
pub fn factoradic_to_permutation(factoradic: &[usize]) -> Permutation {
    let n = factoradic.len();
    let mut available: Vec<usize> = (0..n).collect();
    let mut perm = vec![0; n];
    
    for i in 0..n {
        let index = factoradic[i];
        // Bounds check: factoradic[i] must be < available.len()
        let safe_index = index.min(available.len().saturating_sub(1));
        perm[i] = available.remove(safe_index);
    }
    
    perm
}

/// Convert factoradic to integer
pub fn factoradic_to_int(factoradic: &[usize]) -> u128 {
    let n = factoradic.len();
    let mut result: u128 = 0;
    let mut factorial: u128 = 1;
    
    for i in (0..n).rev() {
        result += factoradic[i] as u128 * factorial;
        factorial *= (n - i) as u128;
    }
    
    result
}

/// Convert integer to factoradic
pub fn int_to_factoradic(mut num: u128, n: usize) -> Factoradic {
    let mut factoradic = vec![0; n];
    
    for i in (0..n).rev() {
        let divisor = factorial(n - i) as u128;
        factoradic[i] = (num / divisor) as usize;
        num %= divisor;
    }
    
    factoradic
}

/// Calculate factorial
fn factorial(n: usize) -> u128 {
    if n <= 1 {
        1
    } else {
        (1..=n as u128).product()
    }
}

/// Generate random permutation from PRF output
pub fn prf_to_permutation(prf_bytes: &[u8], n: usize) -> Permutation {
    // Convert PRF bytes to integer (take first 16 bytes for u128)
    let mut num_bytes = [0u8; 16];
    let len = prf_bytes.len().min(16);
    num_bytes[..len].copy_from_slice(&prf_bytes[..len]);
    let num = u128::from_le_bytes(num_bytes);
    
    // Modulo n! to get valid index
    let max_index = factorial(n);
    let index = num % max_index;
    
    // Convert to factoradic then to permutation
    let factoradic = int_to_factoradic(index, n);
    factoradic_to_permutation(&factoradic)
}

/// Calculate log2(n!) for bit budget estimation
pub fn log2_factorial(n: usize) -> f64 {
    (1..=n).map(|i| (i as f64).log2()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_permutation_to_factoradic() {
        let perm = vec![2, 0, 3, 1];
        let factoradic = permutation_to_factoradic(&perm);
        assert_eq!(factoradic, vec![2, 0, 1, 0]);
    }
    
    #[test]
    fn test_factoradic_to_permutation() {
        let factoradic = vec![2, 0, 1, 0];
        let perm = factoradic_to_permutation(&factoradic);
        assert_eq!(perm, vec![2, 0, 3, 1]);
    }
    
    #[test]
    fn test_round_trip() {
        let original = vec![3, 1, 4, 0, 2];
        let factoradic = permutation_to_factoradic(&original);
        let recovered = factoradic_to_permutation(&factoradic);
        assert_eq!(original, recovered);
    }
    
    #[test]
    fn test_factoradic_to_int() {
        let factoradic = vec![2, 0, 1, 0];
        let num = factoradic_to_int(&factoradic);
        assert_eq!(num, 13);
    }
    
    #[test]
    fn test_int_to_factoradic() {
        let factoradic = int_to_factoradic(13, 4);
        assert_eq!(factoradic, vec![2, 0, 1, 0]);
    }
    
    #[test]
    fn test_factorial() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(4), 24);
        assert_eq!(factorial(5), 120);
    }
    
    #[test]
    fn test_prf_to_permutation() {
        let prf_bytes = [42u8; 32];
        let perm = prf_to_permutation(&prf_bytes, 5);
        
        // Check it's a valid permutation
        assert_eq!(perm.len(), 5);
        let mut sorted = perm.clone();
        sorted.sort();
        assert_eq!(sorted, vec![0, 1, 2, 3, 4]);
    }
    
    #[test]
    fn test_log2_factorial() {
        // log2(4!) = log2(24) ≈ 4.585
        let result = log2_factorial(4);
        assert!((result - 4.585).abs() < 0.01);
        
        // log2(64!) ≈ 295.995
        let result64 = log2_factorial(64);
        assert!((result64 - 295.995).abs() < 0.01);
    }
    
    #[test]
    fn test_identity_permutation() {
        let identity = vec![0, 1, 2, 3];
        let factoradic = permutation_to_factoradic(&identity);
        assert_eq!(factoradic, vec![0, 0, 0, 0]);
        
        let num = factoradic_to_int(&factoradic);
        assert_eq!(num, 0);
    }
    
    #[test]
    fn test_reverse_permutation() {
        let reverse = vec![3, 2, 1, 0];
        let factoradic = permutation_to_factoradic(&reverse);
        assert_eq!(factoradic, vec![3, 2, 1, 0]);
        
        let num = factoradic_to_int(&factoradic);
        assert_eq!(num, 23); // 4! - 1
    }
}
