// Função para realizar a soma de dois números u64 e retornar um
// tipo primitivo (u64).
#[no_mangle]
pub fn add(left: u64, right: u64) -> u64 {
  left + right
}
