pub fn merge_sort(lista: Vec<i32>) -> Vec<i32> {
    if lista.len() <= 1 {
        return lista;
    }
    let meio = lista.len() / 2;
    let esquerda = merge_sort(lista[..meio].to_vec());
    let direita = merge_sort(lista[meio..].to_vec());
    merge(esquerda, direita)
}

fn merge(esquerda: Vec<i32>, direita: Vec<i32>) -> Vec<i32> {
    let mut res = Vec::with_capacity(esquerda.len() + direita.len());
    let (mut i, mut j) = (0, 0);

    while i < esquerda.len() && j < direita.len() {
        if esquerda[i] <= direita[j] {
            res.push(esquerda[i]);
            i += 1;
        } else {
            res.push(direita[j]);
            j += 1;
        }
    }
    res.extend_from_slice(&esquerda[i..]);
    res.extend_from_slice(&direita[j..]);
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge() {
        assert_eq!(merge_sort(vec![3, 1, 2]), vec![1, 2, 3]);
    }
}
