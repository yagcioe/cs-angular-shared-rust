using System;
using uniffi.dotnet_uniffi;

namespace ConsoleApp2
{
  internal class Program
    {
        static void Main(string[] args)
        {
      while (true)
          {
            Console.WriteLine("Enter string to validate...");
            var str = Console.ReadLine();
            if (str == String.Empty) str = null;
            Console.WriteLine($"entered: {str ?? "null"}");
            Console.WriteLine($"valid: {DotnetUniffiMethods.Valid(str)}");
          }
    }
    }
}
