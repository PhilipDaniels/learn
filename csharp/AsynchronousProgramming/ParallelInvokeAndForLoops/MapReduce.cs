using System;
using System.Collections.Generic;
using System.Linq;
using HelperLib;

namespace ParallelForExamples
{
    public static class MapReduce
    {
        public static double CalculateStandardDevitation()
        {
            Log.Enter();
            Log.Msg("Generating random numbers...");

            List<int> values = GenerateNumbers(100 * 1000 * 1000).ToList();
            Log.StartStopwatch();

            double average = values.AsParallel().Average();

            double std = values
                .AsParallel()
                .Aggregate(
                    0.0,
                    // produces local total
                    (subTotal, nextNumber) => subTotal += Math.Pow(nextNumber - average, 2),
                    // sum of all local totals
                    (total, threadTotal) => total += threadTotal,
                    // final projection of the combined results
                    grandTotal => Math.Sqrt(grandTotal / (double)(values.Count - 1))
                    );

            Log.Msg("standard deviation = " + std);
            Log.TimeTaken();

            return std;
        }

        private static IEnumerable<int> GenerateNumbers(int quantity)
        {
            Random rnd = new Random();
            for (int i = 0; i < quantity; i++)
            {
                yield return rnd.Next(1, 100);
            }
        }
    }
}
