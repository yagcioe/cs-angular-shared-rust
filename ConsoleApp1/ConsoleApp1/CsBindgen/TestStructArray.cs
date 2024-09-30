namespace CsBindgen
{
  internal partial struct TestStructArray
  {
    public static unsafe TestStructArray fromArray(TestStruct[] arr)
    {
      fixed (TestStruct* ptr = arr)
      {
        return new TestStructArray { capacity = arr.Length, length = arr.Length,  ptr = ptr };
      }
    }

    public unsafe TestStruct[] toArray()
    {
      var span = new Span<TestStruct>(ptr, length);
      NativeMethods.free_test_arr(this);
      return span.ToArray();
    }
  }

  internal static class TestStructArrayExtensions
  {
    public static TestStructArray ToStructArray(this TestStruct[] arr)
    {
      return TestStructArray.fromArray(arr);
    }
  }
}
