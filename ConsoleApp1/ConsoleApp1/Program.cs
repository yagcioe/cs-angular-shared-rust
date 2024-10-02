Console.WriteLine(uniffi.dotnet_uniffi.DotnetUniffiMethods.Test(12));
Console.WriteLine(uniffi.dotnet_uniffi.DotnetUniffiMethods.TestStr("hello"));
try
{

Console.WriteLine(uniffi.dotnet_uniffi.DotnetUniffiMethods.TestPanic("hello"));
}catch(uniffi.dotnet_uniffi.PanicException ex)
{
  Console.Error.WriteLine(ex.Message);
}
