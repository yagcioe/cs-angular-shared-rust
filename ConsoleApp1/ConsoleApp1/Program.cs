using uniffi.dotnet_uniffi;

Console.WriteLine(DotnetUniffiMethods.Test(12));
Console.WriteLine(DotnetUniffiMethods.TestStr("hello"));
try
{
  DotnetUniffiMethods.TestPanic();
}catch(PanicException ex)
{
  Console.Error.WriteLine(ex.Message);
}

Console.WriteLine(DotnetUniffiMethods.Factorial(10));
Console.WriteLine(DotnetUniffiMethods.Last(new List<long>() { 12,13,14}));
