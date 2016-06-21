using System.Threading;
using System.Threading.Tasks;

namespace AsyncAwaitBestPractices
{
    // See https://msdn.microsoft.com/en-us/magazine/jj991977.aspx
    class Program
    {
        static void Main(string[] args)
        {
            // Name                 Description                                         Exceptions
            // ====                 ===========                                         ==========
            // Avoid async void     Prefer async Task methods over async void methods   Event handlers
            // Async all the way    Don’t mix blocking and async code                   Console main method
            // Configure context    Use ConfigureAwait(false) when you can              Methods that require con­text


            //Example1.DoIt();
            Example2.DoIt();
        }
    }
}
