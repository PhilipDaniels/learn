using System;
using System.Collections.Concurrent;
using System.Diagnostics;
using System.Runtime.CompilerServices;
using System.Threading;
using System.Threading.Tasks;

namespace HelperLib
{
    public static class Log
    {
        static int i = 1;
        static object padlock = new object();
        static ConcurrentDictionary<int, Stopwatch> stopwatches = new ConcurrentDictionary<int, Stopwatch>();

        public static void Enter([CallerMemberName]string caller = "")
        {
            InnerLog(caller, "Enter");
        }

        public static void Exit([CallerMemberName]string caller = "")
        {
            InnerLog(caller, "Exit");
        }

        public static void Msg(string msg, [CallerMemberName]string caller = "")
        {
            InnerLog(caller, msg);
        }

        /// <summary>
        /// Starts a stopwatch which is associated with the current managed thread.
        /// </summary>
        public static void StartStopwatch()
        {
            int tid = Thread.CurrentThread.ManagedThreadId;

            stopwatches.AddOrUpdate(tid,
                (key) =>
                {
                    // This delegate is invoked if the dictionary does not contain an element for the key.
                    var w = new Stopwatch();
                    w.Start();
                    return w;
                },
                (key, existingVal) =>
                {
                    // This delegate is invoked if the dictionary already contains an element for the key.
                    existingVal.Restart();
                    return existingVal;
                });
        }

        /// <summary>
        /// Stops a stopwatch which is associated with the current managed thread.
        /// </summary>
        public static void StopStopwatch()
        {
            int tid = Thread.CurrentThread.ManagedThreadId;
            Stopwatch sw;

            if (!stopwatches.TryRemove(tid, out sw))
            {
                throw new InvalidOperationException("Never started the stopwatch.");
            }

        }

        /// <summary>
        /// Logs the time taken as recorded by a stopwatch which is associated with the current managed thread.
        /// Use <see cref="StartStopwatch"/> to start the stopwatch.
        /// </summary>
        /// <param name="msg"></param>
        /// <param name="caller"></param>
        public static void TimeTaken(string msg = "", [CallerMemberName]string caller = "")
        {
            int tid = Thread.CurrentThread.ManagedThreadId;
            Stopwatch sw;

            if (stopwatches.TryGetValue(tid, out sw))
            {
                InnerLog(caller, "Time taken " + sw.ElapsedMilliseconds + " msec.");
            }
            else
            {
                throw new InvalidOperationException("Never started the stopwatch.");
            }
        }

        private static void InnerLog(string caller, string msg)
        {
            lock (padlock)
            {
                var col = Console.ForegroundColor;
                Console.Write("{0,-3:000} ", i);

                Console.ForegroundColor = ConsoleColor.Cyan;
                Console.Write("{0:mm:ss.ffff} ", DateTime.Now);

                Console.ForegroundColor = ConsoleColor.Red;
                Console.Write("MTID {0,-3} ", Thread.CurrentThread.ManagedThreadId);
                if (Task.CurrentId != null)
                {
                    Console.Write("TaskId {0,-3}", Task.CurrentId);
                }
                else
                {
                    Console.Write("          ");
                }

                Console.ForegroundColor = ConsoleColor.Yellow;
                Console.Write("{0,-15} ", caller);

                Console.ForegroundColor = ConsoleColor.White;
                Console.WriteLine(msg);

                Console.ForegroundColor = col;

                i++;
            }
        }
    }
}
