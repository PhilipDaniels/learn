using System;
using System.Threading.Tasks;
using HelperLib;

namespace AsyncAwaitBestPractices
{
    /// <summary>
    /// This does what I want.
    /// All the Enter messages are printed first as the execution chain goes down the stack.
    /// All the "awaits" result in continuations for the 2nd halves of the methods being registered.
    /// Then we come all the way back up to DoIt(), and "Doing stuff" is printed.
    /// All this happens immediately.
    /// Then we t.Wait() until the 2 second delay expires.
    /// The continuations then unwind (or "deliver"), starting at the innermost, FirstMethod(),
    /// then MainAsync(), then finally DoIt() and we hit the ReadKey() call.
    /// </summary>
    static class Example2
    {
        public static void DoIt()
        {
            Log.Enter();
            var t = MainAsync();
            Log.Msg("Doing stuff");

            t.Wait();

            Log.Exit();
            Console.WriteLine("Press a key...");
            Console.ReadKey();
        }

        public static async Task MainAsync()
        {
            Log.Enter();
            string msg = await FirstMethod();
            Log.Exit();
        }

        public static async Task<string> FirstMethod()
        {
            Log.Enter();
            await Task.Delay(2000);
            Log.Exit();
            return "from first";
        }
    }
}
