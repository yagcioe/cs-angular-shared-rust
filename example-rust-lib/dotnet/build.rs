fn main() {
    csbindgen::Builder::default()
        .input_extern_file("src/lib.rs")
        .csharp_dll_name("my_dotnet_lib")
        .generate_csharp_file("../../ConsoleApp1/ConsoleApp1/NativeMethods.g.cs")
        .unwrap();
}