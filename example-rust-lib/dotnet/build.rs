fn main() {
    csbindgen::Builder::default()
        .input_extern_file("src/lib.rs")
        .input_extern_file("src/string/csharp_string.rs")
        .input_extern_file("src/string/rust_string.rs")
        .input_extern_file("src/test_case/mod.rs")
        .csharp_dll_name("my_dotnet_lib")
        .csharp_type_rename(|name:String| -> String  {
            name
        })
        .generate_csharp_file("../../ConsoleApp1/ConsoleApp1/CsBindgen/NativeMethods.g.cs")
        .unwrap();
}
