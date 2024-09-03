[Read in English](README-en.md)

# :crab: Exemplo de biblioteca dinâmica (cdylib) em Rust

Neste exemplo, será criado duas bibliotecas dinâmicas, a `kernel.dll` e a `utils.dll`.
Dentro da `utils.dll` temos uma função de soma (`add`), que será importada para dentro da
biblioteca `kernel.dll`.

## Usando

**1** - Clone e compile o projeto:

```
git clone https://github.com/williamcanin/rust_libs_examples.git
cd rust-libs-examples
cargo build
```

**2** - Acesse a pasta onde as bibliotecas dinâmicas foram criadas:

```
cd target\debug
```

**3** - Exemplo de usar a biblioteca dinâmica no Windows com o `rundll32.exe`:

```
rundll32.exe kernel.dll, main
```

**4** - No Linux não existe (até onde sei) uma ferramenta igual o `rundll32.exe` para chamar diretamente a biblioteca dinâmica e função dentro da mesma, então você pode criar um programa em `C` ou um `Shell Script` (com código C embutido) para chamar a biblioteca.

Lembrando que no Linux as bibliotecas dinâmicas será com extensão `.so`, o que leva a você ter que mudar a vinculação externas das bibliotecas dinâmicas no código, por exemplo, em vez de `#[link(name = "utils.dll", kind = "dylib")]` você terá que mudar para `#[link(name = "utils", kind = "dylib")]`, sem extensão em `"utils"`.

Exemplo de código em `C` para chamar a biblioteca dinâmica `libkernel.so`:

`setup.c`:

```
#include <stdio.h>
#include <dlfcn.h>

int main() {
    void *handle;
    void (*main_func)();

    handle = dlopen("./target/debug/libkernel.so", RTLD_LAZY);
    if (!handle) {
        fprintf(stderr, "%s\n", dlerror());
        return 1;
    }

    dlerror(); // Limpa erros existentes

    *(void **) (&main_func) = dlsym(handle, "main");
    if (!main_func) {
        fprintf(stderr, "%s\n", dlerror());
        return 1;
    }

    main_func();

    dlclose(handle);
    return 0;
}
```

Compilando e executando o código em `C`:

```
gcc -o setup setup.c -ldl
./runlib
```

Você também pode fazer um `Shell Script` para rodar direto sem precisar compilar, para isso faça:

`setup.sh`:

```
#!/bin/bash

# Nome da biblioteca e função a ser chamada
LIB_NAME="./target/release/libkernel.so"
FUNC_NAME="main"

# Carrega a biblioteca e chama a função
gcc -o setup -ldl -xc - <<EOF
#include <stdio.h>
#include <dlfcn.h>

int main() {
    void *handle;
    void (*func)();

    handle = dlopen("$LIB_NAME", RTLD_LAZY);
    if (!handle) {
        fprintf(stderr, "%s\n", dlerror());
        return 1;
    }

    dlerror(); // Limpa erros existentes

    *(void **) (&func) = dlsym(handle, "$FUNC_NAME");
    if (!func) {
        fprintf(stderr, "%s\n", dlerror());
        return 1;
    }

    func();

    dlclose(handle);
    return 0;
}
EOF

./setup
```

Executando:

```
bash setup.sh
```

Irá ser criado o arquivo `result.txt` com a soma de dois números passado na função
`create_file`.

4 - É isso.

---
(c) - William Canin - [LICENSE](LICENSE)