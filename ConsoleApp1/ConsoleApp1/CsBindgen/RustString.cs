using System.Text;

namespace CsBindgen
{
  internal partial struct RustString
  {
    public unsafe Span<byte> AsSpan()
    {
      return new Span<byte>(ptr, count);
    }

    public void Free()
    {
      NativeMethods.free_u8_string(this);
    }

    public unsafe string moveValue()
    {
      var result = Encoding.UTF8.GetString(AsSpan());
      Free();
      return result;
    }
  }
}
