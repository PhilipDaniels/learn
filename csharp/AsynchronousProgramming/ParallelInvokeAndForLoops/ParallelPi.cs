using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;
using HelperLib;

namespace ParallelForExamples
{
    /// <summary>
    /// Calculate PI using the formula:
    ///         4 * (1 + ( -1.0 / 3.0 + 1.0/5.0 - 1.0/7.0 + 1.0/9.0 - 1.0/11.0 ... ))
    /// </summary>
    public static class ParallelPi
    {
        /// <summary>
        /// This is the single-threaded implementation.
        /// </summary>
        public static double CalculatePiSingleThreaded(int iterations)
        {
            Log.Enter();
            Log.StartStopwatch();

            double pi = 1;
            double multiplier = -1;

            for (int i = 3; i < iterations; i += 2)
            {
                double term = 1.0 / (double)i * multiplier;
                //Log.Msg("i = " + i + " Term = " + term);
                pi += term;
                multiplier *= -1;
            }

            pi *= 4.0;

            Log.Msg("Pi = " + pi);
            Log.TimeTaken();

            return pi;
        }

        /// <summary>
        /// This is an initial attempt at a multi-threaded version. It is essentially a replacement
        /// of the original loop with a Parallel.For. It runs slower and calculates the wrong
        /// answer because the update of shared state is not single-threaded.
        /// </summary>
        public static double CalculatePiIncorrectly(int iterations)
        {
            Log.Enter();
            Log.StartStopwatch();

            double pi = 1;
            double multiplier = -1;

            Parallel.For(0, (iterations - 3) / 2, loopIndex =>
               {
                   int i = 3 + loopIndex * 2;
                   pi += 1.0 / (double)i * multiplier;
                   multiplier *= -1;
               });

            pi *= 4.0;

            Log.Msg("Pi = " + pi);
            Log.TimeTaken();

            return pi;
        }

        ///// <summary>
        ///// This is a fixed attempt. We ensure each loop iteration has only the data that it needs
        ///// to run using a "local init" delegate. That way there is no contention or cross-talk.
        ///// Then we combine the individual results using a thread-safe "local finally" delegate.
        ///// </summary>
        public static double CalculatePiInParallelCorrectly(int iterations)
        {
            Log.Enter();
            Log.StartStopwatch();
            object padlock = new object();

            // We are using:
            //
            // public static ParallelLoopResult For<TLocal>(int fromInclusive, int toExclusive,
            //                                              Func<TLocal> localInit,
            //                                              Func< int,ParallelLoopState, TLocal, TLocal > body,
            //                                              Action<TLocal> localFinally);

            double pi = 1;

            Parallel.For(0, (iterations - 3) / 2,
                // Each term does not need any initial state
                () => 0.0,
                (int loopIndex, ParallelLoopState loopState, double localPi) =>
                {
                    // We are calculating the terms from:
                    //    4 * ( -1.0 / 3.0 + 1.0/5.0 - 1.0/7.0 + 1.0/9.0 - 1.0/11.0 ... )
                    // Note that you need += and not just =. This is because this delegate may be called several times.
                    // See "trash can analogy" at http://www.albahari.com/threading/part5.aspx#_The_Parallel_Class
                    // Each worker has a local trash can which is occasionally emptied into the master trash can.
                    // In other words, we may be asked to calculate SEVERAL terms and add them up by ourselves, before
                    // being asked to add the local total to the final result.
                    double multiplier = loopIndex % 2 == 0 ? -1 : 1;
                    int i = 3 + loopIndex * 2;
                    localPi += 1.0 / (double)i * multiplier;
                    return localPi;
                },
                (double localPi) =>
                {
                    // Here we add all the individual terms together.
                    lock (padlock)
                    {
                        pi += localPi;
                    }
                });

            pi *= 4.0;

            Log.Msg("Pi = " + pi);
            Log.TimeTaken();

            return pi;
        }

        /// <summary>
        /// By nesting one loop inside another, you can increase the amount of work done by each
        /// Task, and therefore reduce the overhead.
        /// </summary>
        public static double CalculatePiInParallelUsingNestedLoops(int iterations)
        {
            Log.Enter();
            Log.StartStopwatch();
            object padlock = new object();

            IEnumerable<Tuple<int, int>> ranges = Range(3, iterations, 10000);
            double pi = 1;
            foreach (var range in ranges)
            {
                double multiplier = range.Item1 % 2 == 0 ? 1 : -1;
                for (int i = range.Item1; i < range.Item2; i += 2)
                {
                    pi += 1.0 / (double)i * multiplier;
                    multiplier *= -1;
                }
            }
            pi *= 4.0;

            Log.Msg("Pi = " + pi);
            Log.TimeTaken();

            return pi;
        }

        private static IEnumerable<Tuple<int, int>> Range(int start, int end, int size)
        {
            for (int i = start; i < end; i += size)
            {
                yield return Tuple.Create(i, Math.Min(i + size, end));
            }
        }

        /// <summary>
        /// Calculates pi in parallel using PLINQ.
        /// </summary>
        public static double CalculatePiInParallelUsingPLINQ(int iterations)
        {
            Log.Enter();
            Log.StartStopwatch();

            var pi = 1 + ParallelEnumerable.Range(0, (iterations - 3) / 2)
                .Sum(loopIndex =>
                {
                    double multiplier = loopIndex % 2 == 0 ? -1 : 1;
                    int i = 3 + loopIndex * 2;
                    double term = multiplier * (1.0 / i);
                    //Log.Msg("loopIndex = " + loopIndex + " i = " + i + " mult = " + multiplier + " term = " + term);
                    return term;
                }
                );

            pi *= 4.0;
            Log.Msg("Pi = " + pi);
            Log.TimeTaken();

            return pi;
        }
    }
}
