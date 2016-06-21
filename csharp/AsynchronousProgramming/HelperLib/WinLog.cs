using System;
using System.Runtime.CompilerServices;
using System.Windows.Forms;

namespace HelperLib
{
    public class WinLog
    {
        private readonly TextBox output;

        public WinLog(TextBox output)
        {
            this.output = output;
        }

        static int i = 1;

        public void Enter([CallerMemberName]string caller = "")
        {
            InnerLog(caller, "Enter");
        }

        public void Exit([CallerMemberName]string caller = "")
        {
            InnerLog(caller, "Exit");
        }

        public void Msg(string msg, [CallerMemberName]string caller = "")
        {
            InnerLog(caller, msg);
        }

        private void InnerLog(string caller, string msg)
        {
            string logMessage = String.Format("{0:00} - {1:mm:ss.ff} - {2,-15} : {3}", i, DateTime.Now, caller, msg);
            output.Text += logMessage + Environment.NewLine;
            i++;
        }
    }
}
