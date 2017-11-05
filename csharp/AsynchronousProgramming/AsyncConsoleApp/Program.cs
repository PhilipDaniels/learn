using System;
using System.Threading.Tasks;

namespace AsyncConsoleApp
{
    class Program
    {
        static void Main(string[] args)
        {
            Console.WriteLine("Time = " + DateTime.Now);
            // async void Main() doesn't work, the program needs to block until
            // all async work has completed. This is the way to do it.
            var result = DoSomethingAsync().GetAwaiter().GetResult();
            Console.WriteLine("Time = " + DateTime.Now);
        }

        static async Task<int> DoSomethingAsync()
        {
            await Task.Delay(TimeSpan.FromSeconds(2));
            var s = await DoSomethingElseAsync();
            return Int32.Parse(s);
        }

        static async Task<string> DoSomethingElseAsync()
        {
            await Task.Delay(TimeSpan.FromSeconds(2));
            return "42";
        }
    }
}
