using System;
using System.Collections.Generic;
using uniffi.dotnet_uniffi;

namespace ConsoleApp2
{
  internal class Program
  {
    static void Main(string[] args)
    {
      Console.WriteLine(DotnetUniffiMethods.GetFactorial(22));
      while (true)
      {
        Console.WriteLine("Enter string to validate...");
        var str = Console.ReadLine();
        if (str == String.Empty) str = null;
        Console.WriteLine($"entered: {str ?? "null"}");
        Console.WriteLine($"valid: {DotnetUniffiMethods.Valid(str)}");
        var results = DotnetUniffiMethods.ComputeAll(new List<ComputationRequest>() { new ComputationRequest(12), new ComputationRequest(12), new ComputationRequest(12) });
        Console.WriteLine(results[0].value);
      }
    }
  }
}
