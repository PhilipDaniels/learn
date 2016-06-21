using HelperLib;

namespace Timers
{
    public static class TimerTimer
    {
        public static void AutoResetExample()
        {
            // Timer will fire only once.
            var t = new System.Timers.Timer(500);
            t.AutoReset = false;
            t.Start();
            t.Elapsed += (sender, args) => { Log.Msg("Timer fired"); };
        }
    }
}
