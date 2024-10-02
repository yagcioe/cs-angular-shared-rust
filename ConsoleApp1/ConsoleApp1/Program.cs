using uniffi.dotnet_uniffi;

while (true)
{
  Console.WriteLine("Enter string to validate...");
  var str = Console.ReadLine();
  if (str == String.Empty) str = null;
  Console.WriteLine($"entered: {str??"null"}");
  Console.WriteLine($"valid: {DotnetUniffiMethods.Valid(str)}");
}
