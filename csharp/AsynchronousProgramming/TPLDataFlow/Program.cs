using System.IO;
using System.Linq;
using System.Threading.Tasks.Dataflow;
using HelperLib;

namespace TPLDataFlow
{
    class Program
    {
        const string InputDir = @"C:\temp\TPL-in";
        const string OutputDir = @"C:\temp\TPL-out";

        static TransformManyBlock<string, Payload> DirectoryReaderBlock;
        static ActionBlock<Payload> TooSmallBlock;

        static void Main(string[] args)
        {
            CreatePipeline();
        }

        private static void CreatePipeline()
        {
            // Give me all the files ...
            DirectoryReaderBlock = new TransformManyBlock<string, Payload>((directory) =>
            {
                var di = new DirectoryInfo(directory);

                return from fi in di.EnumerateFiles("*.*", SearchOption.AllDirectories)
                       select new Payload() { FileInfo = fi };
            });

            TooSmallBlock = new ActionBlock<Payload>((payload) =>
            {
                Log.Msg("Discarding " + payload.FileInfo.FullName + ", size is " + payload.FileInfo.Length);
            });

            DirectoryReaderBlock.LinkTo(TooSmallBlock, p => p.FileInfo.Length < 20 * 1024);
        }
    }
}
