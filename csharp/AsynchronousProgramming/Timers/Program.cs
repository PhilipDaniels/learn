using System;

namespace Timers
{
    class Program
    {
        static void Main(string[] args)
        {
            //ThreadingTimer.SimpleExample();
            //ThreadingTimer.SelfTerminatingExample();
            //ThreadingTimer.SelfTerminatingExample2();
            TimerTimer.AutoResetExample();

            Console.WriteLine("Press any key...");
            Console.ReadKey();
        }


    }
}
