using System.Threading.Tasks;
using System.Windows.Forms;
using HelperLib;

namespace Example1
{
    public partial class Form1 : Form
    {
        private readonly WinLog winLog;

        public Form1()
        {
            InitializeComponent();
            winLog = new WinLog(output);
        }

        public async void DoIt()
        {
            // This method prints "Doing stuff" before the task completes because
            // we do some work (the printing) before awaiting the t.
            winLog.Enter();
            var t = FirstMethod();
            winLog.Msg("Doing stuff");
            string msg = await t;
            winLog.Exit();
        }

        public async void DoItNormalCallStack()
        {
            // This produces a normal looking call stack, because await exits the method!
            // so "Doing stuff" only gets printed when the delay has expired.
            winLog.Enter();
            string msg = await FirstMethod();
            winLog.Msg("Doing stuff");
            winLog.Exit();
        }

        public async Task<string> FirstMethod()
        {
            winLog.Enter();
            await Task.Delay(10 * 1000);
            winLog.Exit();
            return "from first";
        }

        private void button1_Click(object sender, System.EventArgs e)
        {
            DoIt();
            //DoItNormalCallStack();
        }

        private void button2_Click(object sender, System.EventArgs e)
        {
            MessageBox.Show("A dialog message");
        }
    }
}
