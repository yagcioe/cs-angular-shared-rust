using CsBindgen;
var str = "a";
// See https://aka.ms/new-console-template for more information
  var t = new TestStruct
  {
    key = 11,
    value = 1,
    value1 = 2,
    value2 = 3,
    value3 = 4,
    value4 = 5,
    value5 = 6,
    value6 = 7,
    value7 = 8,
    value8 = 9,
    value9 = 10,
    value10 = 11,
    value11 = 12,
    value12 = 13,
    value13 = 14,
    value14 = 15,
    value15 = 16,
    value16 = 17,
    value17 = 18,
    value18 = 19,
  };
var arr = new List<TestStruct> { t }.ToArray();
double expecetdBytes = 0;
double mag = 100;
while (true)
{

  unsafe
  {
    str = NativeMethods.get_csharp_string(str.ToCSharpString()).moveValue();
    NativeMethods.get_test_mut_arr(arr.ToStructArray()).toArray();
    t = NativeMethods.get_test_mut(t);
    expecetdBytes += 80;
  }

  if(expecetdBytes >= mag)
  {
    Console.WriteLine(expecetdBytes/(1024*1024));
    mag = mag * 10;
    Console.WriteLine(arr.ToArray()[0].value1);
  }

  //Console.WriteLine(t.key);
  //Console.WriteLine(str);
  //Console.WriteLine(arr[0].key);
}

