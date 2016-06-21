using System;
using System.Collections.Concurrent;
using System.Threading;
using System.Threading.Tasks;
using HelperLib;
using BC = System.Collections.Concurrent.BlockingCollection<ProducerConsumerQueue.WorkItem>;

namespace ProducerConsumerQueue
{
    class Program
    {
        static Random random = new Random();

        static void Main(string[] args)
        {
            DoIt();
        }

        public static void DoIt()
        {
            // Without a capacity the queue is unbounded, which means the producer will add items
            // as fast as it can, potentially leading to out of memory errors. To demonstrate,
            // add a 100Kb byte array to the WorkItem and increase the number of items that you
            // add to the queue. The fix is easy - add a capacity as the second parameter of the
            // constructor, i.e. on the next line.
            var queue = new BC(new ConcurrentQueue<WorkItem>());

            // Producer needs to run on its own thread.
            var producer = Task.Run(() => Producer(queue));

            // N Consumers will all run on their own threads as well, of course.
            Task[] consumers = new Task[4];
            for (int i = 0; i < consumers.Length; i++)
            {
                consumers[i] = Task.Run(() => Consumer(queue));
            }

            // Must wait for everything to finish.
            Log.Msg("Waiting for completion");
            producer.Wait();
            Task.WaitAll(consumers);

            Log.Msg("Press a key");
            Console.ReadKey();
        }

        public static void Producer(BC queue)
        {
            for (int i = 0; i < 10; i++)
            {
                var wi = new WorkItem() { Id = i, Workload = random.Next(0, 2000) };
                queue.Add(wi);
                Log.Msg("Enqueued WorkItem " + wi.Id + " with workload of " + wi.Workload);
            }

            // If this call is omitted the program will not finish. The *producer* will finish,
            // but all the consumers will never finish because they will think that there is more
            // work coming. Calling CompleteAdding() signals to the queue that there is no more
            // work, and thence GetConsumingEnumerable() will terminate instead of blocking.
            queue.CompleteAdding();
        }

        public static void Consumer(BC queue)
        {
            foreach (var wi in queue.GetConsumingEnumerable())
            {
                Log.Msg("De-queued WorkItem " + wi.Id + " with workload of " + wi.Workload);
                // Sleep on this thread to simulate doing busy work.
                Thread.Sleep(wi.Workload);
            }
        }
    }
}
