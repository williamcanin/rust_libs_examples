[Read in Portuguese Brazil](README.md)

# :crab: Example of a dynamic library (cdylib) in Rust

In this example, two dynamic libraries will be created, `kernel.dll` and `utils.dll`.
Inside `utils.dll` we have an addition function (`add`), which will be imported into the `kernel.dll` library.

## Usage

**1** - Clone and compile the project:

```
git clone https://github.com/williamcanin/rust_libs_examples.git
cd rust_libs_examples
cargo build
```

**2** - Access the folder where the dynamic libraries were created:

```
cd target\debug
```

**3** - Example of using the dynamic library on Windows with `rundll32.exe`:

```
rundll32.exe kernel.dll, main
```

**4** - On Linux there is no tool (as far as I know) like `rundll32.exe` to directly call the dynamic library and function within it, so you can create a `C` program or a `Shell Script` (with embedded C code) to call the library.

Remembering that in Linux the dynamic libraries will have the extension `.so`, which means you will have to change the external linking of the dynamic libraries in the code, for example, instead of `#[link(name = "utils.dll", kind = "dylib")]` you will have to change it to `#[link(name = "utils", kind = "dylib")]`, without the extension in `"utils"`.

Example `C` code to call the `libkernel.so` dynamic library:

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

    dlerror(); // Cleans existing errors

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

Compiling and running `C` code:

```
gcc -o setup setup.c -ldl
./runlib
```

You can also make a `Shell Script` to run directly without having to compile, to do this do:

`setup.sh`:

```
#!/bin/bash

# Name of the library and function to be called
LIB_NAME="./target/release/libkernel.so"
FUNC_NAME="main"

# Load the library and call the function
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

    dlerror(); // Cleans existing errors

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

Running:

```
bash setup.sh
```

The `result.txt` file will be created with the sum of two numbers passed in the
`create_file` function.

**5** - That's it, the end.

---
(c) - William Canin - [LICENSE](..\LICENSE)