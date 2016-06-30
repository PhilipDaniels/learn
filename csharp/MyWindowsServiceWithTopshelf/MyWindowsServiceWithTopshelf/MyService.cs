using System;
using System.Threading;
using System.Threading.Tasks;

namespace MyWindowsServiceWithTopshelf
{
    public class MyService
    {
        // We use a cancellation token to ensure that the service stops in a well-behaved manner.
        private readonly CancellationTokenSource cancellationTokenSource;
        private readonly CancellationToken cancellationToken;

        // We have one job to run, we could of course have many.
        private Task job;

        public MyService()
        {
            cancellationTokenSource = new CancellationTokenSource();
            cancellationToken = cancellationTokenSource.Token;
        }

        public void Start()
        {
            job = Task.Run(() => DoWork(), cancellationToken);
        }

        public void Stop()
        {
            // Signal the task to stop.
            cancellationTokenSource.Cancel();

            // Then wait for the job. Could have a timeout here.
            job.Wait();
        }

        public void DoWork()
        {
            while (!cancellationToken.IsCancellationRequested)
            {
                Console.WriteLine("I am working");

                Console.WriteLine("   Step 1");
                Thread.Sleep(1000);

                Console.WriteLine("   Step 2");
                Thread.Sleep(1000);

                Console.WriteLine("   Step 3");
                Thread.Sleep(1000);

                Console.WriteLine("   Step 4");
                Thread.Sleep(1000);

                Console.WriteLine("   Step 5");
                Thread.Sleep(1000);
            }
        }
    }
}
