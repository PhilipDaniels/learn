using System;
using System.Threading;
using System.Threading.Tasks;
using HelperLib;

namespace Primitives
{
    /// <summary>
    /// Nightclub analogy: a semaphore is like a club that only allows N people to enter (i.e. threads to run).
    /// Semaphore and SemaphoreSlim are functionally equivalent, the difference being that the latter is
    /// purely managed but cannot be used for cross-process signalling.
    /// NOTE: NORMALLY YOU SHOULD USE SemaphoreSlim as it is much faster.
    /// </summary>
    public static class SemaphoreSlimExample
    {
        static SemaphoreSlim semaphore;

        public static void WaitForEventAndDoWork()
        {
            while (true)
            {
                //semaphore.Wait();
                Log.Msg("The SemaphoreSlim was set, doing work.");
            }
        }

        public static void Run()
        {
            Log.Enter();

            // Capacity of 3...
            semaphore = new SemaphoreSlim(3);
            Log.Msg("Count is now " + semaphore.CurrentCount);

            // But let's create 5 tasks that are all waiting on the semaphore.
            // Only 3 will start.
            Task.Run(() => WaitForEventAndDoWork());
            Task.Run(() => WaitForEventAndDoWork());
            Task.Run(() => WaitForEventAndDoWork());
            Task.Run(() => WaitForEventAndDoWork());
            Task.Run(() => WaitForEventAndDoWork());


            // Put this thread to sleep for a while, then signal the event.
            // This will result in ONE MORE TASK BEING RELEASED (try running in the debugger to prove it).
            Thread.Sleep(2000);
            semaphore.Release();
            Log.Msg("Count is now " + semaphore.CurrentCount);

            Thread.Sleep(2500); // Wait for messages to appear.

            Console.WriteLine("Press any key...");
            Console.ReadKey();
        }
    }
}
