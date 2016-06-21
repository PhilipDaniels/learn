using System.Threading;
using HelperLib;

namespace Timers
{
    public static class ThreadingTimer
    {
        public static void SelfTerminatingExample()
        {
            // Need two variables to be able to refer to 't' within the delegate.
            int count = 10;
            Timer t = null;
            Timer t2 = new Timer((x) =>
            {
                Log.Msg("The timer fired, count is " + count);
                count--;
                if (count == 0)
                {
                    t.Dispose();
                }
            },
            null, 0, 500
            );
            t = t2;
        }

        public static void SelfTerminatingExample2()
        {
            // Makes more sense with a class variable.
            int count = 10;
            timer = new Timer((x) =>
            {
                Log.Msg("The timer fired, count is " + count);
                count--;
                if (count == 0)
                {
                    timer.Dispose();
                }
            },
            null, 0, 500
            );
        }

        static Timer timer;

        public static void SimpleExample()
        {
            // Excutes on a Thread Pool thread.
            var t = new Timer((x) =>
            {
                Log.Msg("The timer fired");
            },
            null, 0, 500
            );
        }
    }
}
