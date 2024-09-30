using System;
using System.Collections.Generic;
using System.Linq;
using System.Runtime.CompilerServices;
using System.Text;
using System.Threading.Tasks;

namespace CsBindgen
{
  internal static class CSharpStringExtensions
  {
    public static unsafe CSharpString ToCSharpString(this string value)
    {
      fixed (char* p = value)
      {
        return new CSharpString
        {
          count = value.Length,
          ptr = (ushort*)p,
        };
      }
    }
  }
}
