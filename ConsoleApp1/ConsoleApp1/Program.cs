using uniffi.dotnet_uniffi;

while (true)
{
  Console.WriteLine("Enter string to validate...");
  var str = Console.ReadLine()??"";
  Console.WriteLine($"valid: {DotnetUniffiMethods.Valid(str)}");
}
