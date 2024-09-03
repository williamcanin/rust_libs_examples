use tokio::{self, io::AsyncWriteExt};

// Realiza o import da função "add" da biblioteca dinâmica "utils.dll"
// Nota: Esta função está usando apenas tipo primitivo no parâmetro e retorno, o que faz com
// que não precisamos fazer usar código C, mas caso queira usar tipos não primitivos
// terá que escrever código em C.
#[link(name = "utils.dll", kind = "dylib")]
extern "C" {
  pub fn add(left: u64, right: u64) -> u64;
}

// Função para criação do arquivo result.txt de forma assíncrona com tokio.
async fn create_file(
  filename: &str,
  left: u64,
  right: u64,
) -> Result<(), Box<dyn std::error::Error>> {
  // Rodando a função externa (add) em bloco unsafe, já que é uma FFi
  let sum: u64 = unsafe { add(left, right) };
  // Usando tokio para criar arquivo
  let mut file = tokio::fs::File::create(filename).await?;
  file.write_all(format!("Sum is: {sum}").as_bytes()).await?;

  Ok(())
}

// Cria uma função main marcando a mesma como "no_mangle" para seu nome não ser
// mudado ao charmar por outro programa.
// Também assíncronismo (tokio) pois está chamando outra função assíncrona, que é "create_file".
#[no_mangle]
#[tokio::main]
async fn main() {
  // Esperando 5 segundos antes de executar a função "create_file"
  tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

  // Chama a função "create_file"
  create_file("result.txt", 22, 20).await.unwrap_or(());
}
