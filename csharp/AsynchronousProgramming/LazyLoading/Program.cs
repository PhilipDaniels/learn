using System;
using System.Threading.Tasks;
using HelperLib;

namespace LazyLoading
{
    class Program
    {
        static void Main(string[] args)
        {
            // By default, lazy is thread-safe which means a lock is taken to guard the creation
            // of the object. There are ctor options to relax this (see Pro, Blewett) but their
            // use cases appear to be very rare.

            //SimpleExample();
            //MultiThreadExample();
            CreationWithNonDefaultConstructor();

            Console.WriteLine("Press any key...");
            Console.ReadKey();
        }

        private static void CreationWithNonDefaultConstructor()
        {
            // If your type does not have a public parameterless ctor then you must use a delegate
            // to initialize the object. This can be a factory method if the delegate gets too long
            // to include inlne.
            Lazy<NamedPerson> lazyPerson = new Lazy<NamedPerson>(() => new NamedPerson("Phil"));
            var p = lazyPerson.Value;
        }

        private static void SimpleExample()
        {
            Lazy<Person> lazyPerson = new Lazy<Person>();

            // First reference to the .Value property creates the person object.
            // If you never refer to it, it is never created.
            // The creation is AUTOMATICALLY thread safe.
            Person p = lazyPerson.Value;
        }

        private static void MultiThreadExample()
        {
            Lazy<Person> lazyPerson = new Lazy<Person>();

            // Spawn multiple Tasks, each of which tries to access the .Value.
            // The person will only be created once.
            var tasks = new Task[4];
            for (int i = 0; i < tasks.Length; i++)
            {
                tasks[i] = Task.Run(() =>
                {
                    Log.Msg("Starting task to access the person.");
                    var p = lazyPerson.Value;
                    Log.Msg("Finished task to access the person.");
                }
                );
            }

            Task.WaitAll(tasks);
        }
    }
}
