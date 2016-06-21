using System;
using System.Threading;
using System.Threading.Tasks;

namespace HelperLib
{
    public static class Jobs
    {
        static Random random = new Random();

        /// <summary>
        /// Delays the current thread by the specified msec. Logs a message too.
        /// </summary>
        /// <param name="msec">The msec.</param>
        /// <param name="info">The information message.</param>
        public static void Delay(int msec, string info = "")
        {
            Log.Msg("Delaying for " + msec + " msec. " + info);
            Thread.Sleep(msec);
        }

        /// <summary>
        /// Delays the current thread by a random number of msec. Logs a message too.
        /// </summary>
        /// <param name="info">The information.</param>
        /// <param name="min">The minimum.</param>
        /// <param name="max">The maximum.</param>
        /// <returns>The number of msec the thread was delayed for.</returns>
        public static int RandomDelay(string info, int min = 0, int max = 2000)
        {
            /* Note that this is wrong, it causes all the jobs to execute sequentially because the sleep is in the lock.
            lock (random)
            {
                int n = random.Next(0, 2000);
                n = 500;
                Log.Msg("Delaying for " + n + " msec");
                Thread.Sleep(n);
            }
            */

            int n = NextRandomInt(min, max);
            Delay(n, info);
            return n;
        }

        /// <summary>
        /// Puts the current thread into a busy-loop for a random number of milliseconds.
        /// </summary>
        public static int BusyLoop(string info, int min = 0, int max = 2000)
        {
            int n = NextRandomInt(min, max);
            Task.Run(() => { Thread.Sleep(n); flag = false; });
            while (flag)
            {
            }

            Delay(n, info);

            return n;
        }

        static volatile bool flag = true;

        /// <summary>
        /// Gets a random int between the specified values, in a thread-safe manner.
        /// </summary>
        public static int NextRandomInt(int min = 0, int max = 2000)
        {
            // Here we lock just enough to protect the non-thread-safe Random class.
            lock (random)
            {
                int n = random.Next(min, max);
                return n;
            }
        }
    }
}
