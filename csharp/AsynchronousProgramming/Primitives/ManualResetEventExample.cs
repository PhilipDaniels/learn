using System;
using System.Threading;
using System.Threading.Tasks;
using HelperLib;

namespace Primitives
{
    /// <summary>
    /// An AutoResetEvent is a good "don't do anything until I tell you it is ready" construct.
    /// Gate analogy: any number of people can queue at the gate, when we call Set() they all get to go through.
    /// NOTE: NORMALLY YOU SHOULD USE ManualResetEventSlim as it is much faster.
    /// See http://www.albahari.com/threading/part2.aspx#_Signaling_with_Event_Wait_Handles
    /// You only need to use ManualResetEvent if you need an OS object for cross-process or cross-domain
    /// signalling.
    /// </summary>
    public static class ManualResetEventExample
    {
        static ManualResetEvent manual;

        public static void WaitForEventAndDoWork()
        {
            while (true)
            {
                manual.WaitOne();
                Log.Msg("The ManualResetEvent was set, doing work.");
            }
        }

        public static void Run()
        {
            Log.Enter();

            manual = new ManualResetEvent(false);

            // Let's create 3 tasks that are all waiting on the event.
            Task.Run(() => WaitForEventAndDoWork());
            Task.Run(() => WaitForEventAndDoWork());
            Task.Run(() => WaitForEventAndDoWork());

            // Put this thread to sleep for a while, then signal the event.
            // This will result in MANY LINES being printed to the console by each task.
            // In other words, the event remains "set" until reset (manually, hence its name).
            Thread.Sleep(2000);
            manual.Set();

            Thread.Sleep(2500); // Wait for messages to appear.

            // Threads will stop now, because they will be blocking on WaitOne().
            manual.Reset();

            Console.WriteLine("Press any key...");
            Console.ReadKey();
        }
    }
}
