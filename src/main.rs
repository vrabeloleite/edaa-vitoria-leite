pub mod ex01_verificar_primeiro;
pub mod ex02_somar_lista;
pub mod ex03_busca_binaria;
pub mod ex04_pares_com_soma;
pub mod ex05_imprimir_pares_e_pares;
pub mod ex06_potencias_de_dois;
pub mod ex07_fibonacci_recursivo;
pub mod ex08_ordenacao_bolha;
pub mod ex09_produto_de_matrizes;
pub mod ex10_merge_sort;
mod aula06;

fn main() {
    println!("--- TRABALHO DE EDAA - ALGORITMOS EM RUST ---");
    
    // Ex 01
    let l1 = [10, 20, 30];
    println!("Ex 01 - Primeiro elemento: {:?}", ex01_verificar_primeiro::verificar_primeiro(&l1));
    
    // Ex 02
    println!("Ex 02 - Soma da lista: {}", ex02_somar_lista::somar_lista(&l1));
    
    // Ex 03
    println!("Ex 03 - Busca Binaria (alvo 20): {:?}", ex03_busca_binaria::busca_binaria(&l1, 20));
    
    // Ex 04
    println!("Ex 04 - Pares com soma 50:");
    ex04_pares_com_soma::pares_com_soma(&[10, 20, 30, 40], 50);
    
    // Ex 05
    println!("Ex 05 - Imprimir pares:");
    ex05_imprimir_pares_e_pares::imprimir_pares_e_pares(&[1, 2]);
    
    // Ex 06
    println!("Ex 06 - Potencias de 2 ate 15:");
    ex06_potencias_de_dois::potencias_de_dois(15);
    
    // Ex 07
    println!("Ex 07 - Fibonacci de 6: {}", ex07_fibonacci_recursivo::fibonacci_recursivo(6));
    
    // Ex 08
    let mut l2 = [5, 2, 9, 1];
    ex08_ordenacao_bolha::ordenacao_bolha(&mut l2);
    println!("Ex 08 - Bubble Sort resultado: {:?}", l2);
    
    // Ex 09
    let mat_a = vec![vec![1, 2], vec![3, 4]];
    let mat_b = vec![vec![2, 0], vec![1, 2]];
    println!("Ex 09 - Produto de Matrizes: {:?}", ex09_produto_de_matrizes::produto_de_matrizes(&mat_a, &mat_b));
    
    // Ex 10
    let l3 = vec![4, 1, 3, 2];
    println!("Ex 10 - Merge Sort resultado: {:?}", ex10_merge_sort::merge_sort(l3));
}
