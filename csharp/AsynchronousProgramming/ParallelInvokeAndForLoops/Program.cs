using System;
using System.Linq;
using System.Threading.Tasks;
using HelperLib;

namespace ParallelForExamples
{
    public static class Program
    {
        static void Main(string[] args)
        {
            //ParallelInvokeExample();

            // Both Parallel.For and Parallel.ForEach can process elements from the input sequence
            // in any order. They both use TPL Tasks to do work.
            ParallelForEachExample();
            ParallelForEachWithBusyLoop();

            const int iterations = 100 * 1000 * 1000;
            ParallelPi.CalculatePiSingleThreaded(iterations);
            ParallelPi.CalculatePiIncorrectly(iterations);
            ParallelPi.CalculatePiInParallelCorrectly(iterations);
            ParallelPi.CalculatePiInParallelUsingNestedLoops(iterations);
            ParallelPi.CalculatePiInParallelUsingPLINQ(iterations);

            MapReduce.CalculateStandardDevitation();

            // Things not covered: configuring PLINQ with WithDegreeOfParallelism() etc.,
            // cancelling parallel queries.

            Console.WriteLine("Press a key...");
            Console.ReadKey();
        }

        /// <summary>
        /// Run 3 jobs simultaneously, get the result from each one.
        /// ParallelInvoke finishes when all three have finished, so if one task waits
        /// forever your program will hang here. To solve this, use can use a CancellationToken,
        /// but it needs to be flowed to all the tasks.
        /// </summary>
        public static void ParallelInvokeExample()
        {
            int x = 0, y = 0, z = 0;

            Parallel.Invoke(
                () => x = Jobs.RandomDelay("job x"),
                () => y = Jobs.RandomDelay("job y"),
                () => z = Jobs.RandomDelay("job z")
                );

            int total = x + y + z;
            Log.Msg("Total = " + total);
        }

        /// <summary>
        /// Parallel.ForEach and Parallel.For are essentially identical in behaviour.
        /// </summary>
        public static void ParallelForEachExample()
        {
            var nums = Enumerable.Range(0, 20);

            // Note that this is a max hint, you are by no means guaranteed to have exactly 10 threads running.
            var opts = new ParallelOptions() { MaxDegreeOfParallelism = 10 };

            Parallel.ForEach(nums, opts, (i) => Jobs.RandomDelay("Element " + i.ToString()));
        }

        /// <summary>
        /// This will send the CPU to 100%.
        /// </summary>
        public static void ParallelForEachWithBusyLoop()
        {
            var nums = Enumerable.Range(0, 20);

            // Note that this is a max hint, you are by no means guaranteed to have exactly 10 threads running.
            var opts = new ParallelOptions() { MaxDegreeOfParallelism = 10};

            Parallel.ForEach(nums, opts, (i) => Jobs.BusyLoop("Element " + i.ToString()));
        }
    }
}
