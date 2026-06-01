pub fn somar_lista(lista: &[i32]) -> i32 {
    let mut total = 0;
    for &elemento in lista {
        total += elemento;
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_somar_lista() {
        assert_eq!(somar_lista(&[1, 2, 3, 4, 5]), 15);
    }
}
