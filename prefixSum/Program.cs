

class Program
{

  public static float[] prefixSum(float[] arr)
  {

    int n = arr.Length;
    float[] prefixSum = new float[n];

    prefixSum[0] = arr[0];

    for (int i = 1; i < n; i++)
    {
      prefixSum[i] = prefixSum[i - 1] + arr[i];
    }

    return prefixSum;
  }
  static void Main()
  {
    float[] arr = { 10.0f, 15.2f, 15.4f, 13.8f };
    float[] prefixSumResult = prefixSum(arr);
    Console.WriteLine(string.Join(", ", prefixSumResult));
  }
}