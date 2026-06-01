pub fn verificar_primeiro(lista: &[i32]) -> Option<i32> {
    if lista.is_empty() {
        None
    } else {
        Some(lista[0])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verificar_primeiro() {
        assert_eq!(verificar_primeiro(&[10, 20, 30]), Some(10));
        assert_eq!(verificar_primeiro(&[]), None);
    }
}
