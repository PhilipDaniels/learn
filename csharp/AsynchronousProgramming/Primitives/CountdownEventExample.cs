using System;
using System.Threading;
using System.Threading.Tasks;
using HelperLib;

namespace Primitives
{
    public static class CountdownEventExample
    {
        static CountdownEvent countdown;

        public static void WaitForEventAndDoWork()
        {
            while (true)
            {
                countdown.Wait();
                Log.Msg("The CountdownEvent was set, doing work.");
            }
        }

        public static void Run()
        {
            Log.Enter();

            // Set the initial count when you create it.
            countdown = new CountdownEvent(3);

            // Let's create 3 tasks that are all waiting on the event.
            Task.Run(() => WaitForEventAndDoWork());
            Task.Run(() => WaitForEventAndDoWork());
            Task.Run(() => WaitForEventAndDoWork());

            // Put this thread to sleep for a while, then signal the event.
            // Nothing will happen because the count has not reached zero yet.
            Thread.Sleep(2000);
            countdown.Signal();     // count = 2
            Log.Msg("Count is now " + countdown.CurrentCount);

            Thread.Sleep(2000);
            countdown.Signal();     // count = 1
            Log.Msg("Count is now " + countdown.CurrentCount);

            Thread.Sleep(2000);
            countdown.Signal();     // count = 0, messages start to appear
            Log.Msg("Count is now " + countdown.CurrentCount);

            Thread.Sleep(1000);     // Wait for messages to appear.

            // Puts the count back up to 3.
            // Threads will stop now, because they will be blocking on WaitOne().
            countdown.Reset();
            Log.Msg("Count is now " + countdown.CurrentCount);

            Console.WriteLine("Press any key...");
            Console.ReadKey();
        }
    }
}
