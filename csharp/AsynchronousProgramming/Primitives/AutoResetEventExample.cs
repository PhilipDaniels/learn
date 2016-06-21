using System;
using System.Threading;
using System.Threading.Tasks;
using HelperLib;

namespace Primitives
{
    /// <summary>
    /// An AutoResetEvent is a good "don't do anything until I tell you it is ready" construct.
    /// Gate analogy: a turnstile. People are queued by calling WaitOne().
    /// ANYBODY can call Set() to allow ONE person through.
    /// The object will then block again the next time WaitOne() is called (i.e. it automatically
    /// resets, which is why it is called AutoResetEvent. The ManualResetEvent does not reset
    /// automatically, you have to call Reset() manually, hence its name.
    /// </summary>
    public static class AutoResetEventExample
    {
        static AutoResetEvent auto;

        public static void WaitForEventAndDoWork()
        {
            while (true)
            {
                auto.WaitOne();
                Log.Msg("The AutoResetEvent was set, doing work.");
            }
        }

        public static void Run()
        {
            Log.Enter();

            auto = new AutoResetEvent(false);

            // Let's create 3 tasks that are all waiting on the event.
            Task.Run(() => WaitForEventAndDoWork());
            Task.Run(() => WaitForEventAndDoWork());
            Task.Run(() => WaitForEventAndDoWork());

            // Put this thread to sleep for a while, then signal the event.
            // This will result in one line being printed to the console.
            Thread.Sleep(2000);
            auto.Set();

            // Each subsequent call will allow some other task (or maybe the same task, we don't
            // know) to proceed.
            Thread.Sleep(500);  // Wait for messages to appear.
            Log.Msg("About to call twice...");
            Thread.Sleep(2000);
            auto.Set();
            auto.Set();

            Thread.Sleep(2500); // Wait for messages to appear.
            Console.WriteLine("Press any key...");
            Console.ReadKey();
        }
    }
}
