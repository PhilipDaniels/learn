using System;
using System.Threading.Tasks;
using HelperLib;

namespace AsyncAwaitBestPractices
{
    static class Example1
    {
        /// <summary>
        /// IN A CONSOLE PROGRAM
        /// This does not work as you expect. The program will terminate without every hitting ReadKey.
        /// This is because "await" examines the awaitable (Task) returned by FirstMethod() and checks
        /// to see if it has completed. It hasn't. Therefore, it turns the reset of the DoIt() method
        /// into a continuation (to be resumed later) and then exits from DoIt(). However, DoIt() is
        /// at the top of the call stack so the program exits without ever hitting the breakpoint.
        /// IN A WEB PROGRAM, THIS WORKS OK IF DOIT() IS ACTUALLY AN EVENT HANDLER.
        /// </summary>
        public static async void DoIt()
        {
            Log.Enter();
            string msg = await FirstMethod();
            Log.Exit();


            Console.WriteLine("Press a key...");
            Console.ReadKey();
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
