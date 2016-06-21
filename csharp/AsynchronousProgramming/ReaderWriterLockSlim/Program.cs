using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading;
using System.Threading.Tasks;
using HelperLib;

namespace ReaderWriterLockSlimExample
{
    class Program
    {
        // The resource we want to protect with a lock.
        static Dictionary<string, string> resource = new Dictionary<string, string>();
        static ReaderWriterLockSlim padlock = new ReaderWriterLockSlim();

        const int NumReaders = 20;
        const int NumWriters = 3;
        const int NumLockAttempts = 1000 * 1000;

        static void Main(string[] args)
        {
            // There is no real difference in this example.

            resource["Mercury"] = "innermost";
            resource["Pluto"] = "outermost";

            TimeUsingMonitors();
            TimeUsingReaderWriterLockSlim();

            Console.WriteLine("Press any key...");
            Console.ReadKey();
        }

        private static void TimeUsingMonitors()
        {
            Log.Enter();
            Log.StartStopwatch();

            // Create some readers.
            var readers = Enumerable.Range(0, NumReaders).Select(i =>
            {
                return Task.Run(() =>
                {
                    for (int j = 0; j < NumLockAttempts; j++)
                    {
                        lock (resource)
                        {
                            string s = resource["Mercury"];
                        }
                    }
                });
            }).ToList();

            // Create some writers.
            var writers = Enumerable.Range(0, NumWriters).Select(i =>
            {
                return Task.Run(() =>
                {
                    for (int j = 0; j < NumLockAttempts; j++)
                    {
                        lock (resource)
                        {
                            resource["Mercury"] = "the innermost planet.";
                        }
                    }
                });
            }).ToList();

            var all = readers;
            all.AddRange(writers);
            Task.WaitAll(all.ToArray());

            Log.TimeTaken();
            Log.Exit();
        }


        private static void TimeUsingReaderWriterLockSlim()
        {
            Log.Enter();
            Log.StartStopwatch();

            // Create some readers.
            var readers = Enumerable.Range(0, NumReaders).Select(i =>
            {
                return Task.Run(() =>
                {
                    for (int j = 0; j < NumLockAttempts; j++)
                    {
                        padlock.EnterReadLock();
                        string s = resource["Mercury"];
                        padlock.ExitReadLock();
                    }
                });
            }).ToList();

            // Create some writers.
            var writers = Enumerable.Range(0, NumWriters).Select(i =>
            {
                return Task.Run(() =>
                {
                    for (int j = 0; j < NumLockAttempts; j++)
                    {
                        padlock.EnterWriteLock();
                        resource["Mercury"] = "the innermost planet.";
                        padlock.ExitWriteLock();
                    }
                });
            }).ToList();

            var all = readers;
            all.AddRange(writers);
            Task.WaitAll(all.ToArray());

            Log.TimeTaken();
            Log.Exit();
        }

    }
}
