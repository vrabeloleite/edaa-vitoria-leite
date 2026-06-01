pub fn potencias_de_dois(n: u64) -> Vec<u64> {
    let mut i: u64 = 1;
    let mut registradas = Vec::new();
    while i < n {
        println!("{}", i);
        registradas.push(i);
        i *= 2;
    }
    registradas
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_potencias_de_dois() {
        assert_eq!(potencias_de_dois(10), vec![1, 2, 4, 8]);
    }
}
