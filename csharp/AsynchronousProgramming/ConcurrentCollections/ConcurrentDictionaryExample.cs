using System;
using System.Collections.Concurrent;
using System.Linq;
using System.Threading.Tasks;

namespace ConcurrentCollections
{
    public static class ConcurrentDictionaryExample
    {
        static Random random = new Random();
        static ConcurrentDictionary<int, string> dict = new ConcurrentDictionary<int, string>();

        public static void Run()
        {
            // The API of the concurrent collections (ConcurrentDictionary, ConcurrentQueue,
            // ConcurrentStack and ConcurrentBag) is designed to be atomic. Therefore, the API
            // differs from the standard collection classes. We have TryAdd, GetOrAdd etc. which
            // have atomic semantics so that you do not have TOCTOU issues which you would have if
            // you had to do things like if (dict.Count() == 0) { dict.Add() }...

            // The locking within the collections is optimised - it uses a lock per-slot in the
            // internal hash table. This is much better than you can do with an external lock
            // around a normal dictionary (Pro, Blewett, p103).

            // Note that the HelperLib.Log class contains an example of a ConcurrentDictionary that
            // is used to manage stopwatches.

            /* For example, here is ConcurrentDictionary:

               public TValue AddOrUpdate(TKey key, Func<TKey, TValue> addValueFactory, Func<TKey, TValue, TValue> updateValueFactory);
               public TValue AddOrUpdate(TKey key, TValue addValue, Func<TKey, TValue, TValue> updateValueFactory);
               public bool ContainsKey(TKey key);
               public TValue GetOrAdd(TKey key, Func<TKey, TValue> valueFactory);
               public TValue GetOrAdd(TKey key, TValue value);
               public bool TryAdd(TKey key, TValue value);
               public bool TryGetValue(TKey key, out TValue value);
               public bool TryRemove(TKey key, out TValue value);
               public bool TryUpdate(TKey key, TValue newValue, TValue comparisonValue);
            */

            // Make 5 adding tasks, which each attempt to add 20 times.
            var adders = Enumerable.Range(0, 5).Select(i =>
            {
            return Task.Run(() =>
            {
                for (int j = 0; j < 20; j++)
                {
                    int r = random.Next(1, 20);
                    // The asterisks will count how may extra attempts were made to add this key.
                    dict.AddOrUpdate(r, "Attempts *", (k, v) => v + "*");
                }
            });
            }).ToArray();


            Task.WaitAll(adders);
            string total = "";
            foreach (var k in dict.Keys.OrderBy(n => n))
            {
                string v = dict[k];
                Console.WriteLine("{0,-2} {1}", k, v);
                total += v.Replace("Attempts ", "");
            }

            Console.WriteLine("Total updates = " + total.Length);
        }
    }
}
