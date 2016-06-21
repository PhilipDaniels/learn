using System.Collections.Generic;
using System.IO;
using System.Reflection;

namespace HelperLib
{
    /// <summary>
    /// Helps with creating input data in a known location.
    /// </summary>
    public static class DataHelper
    {
        public static string ImagesDirectory
        {
            get
            {
                return Path.Combine(Path.GetDirectoryName(Assembly.GetEntryAssembly().Location), "Images");
            }
        }

        public static void WriteImagesToDisk()
        {
            const string prefix = "HelperLib.Images.";

            var asm = Assembly.GetExecutingAssembly();
            foreach (var name in asm.GetManifestResourceNames())
            {
                if (name.StartsWith(prefix))
                {
                    if (!Directory.Exists(ImagesDirectory))
                        Directory.CreateDirectory(ImagesDirectory);

                    string filename = Path.Combine(ImagesDirectory, name.Replace(prefix, ""));
                    using (var imageStream = asm.GetManifestResourceStream(name))
                    using (var fs = new FileStream(filename, FileMode.Create))
                    {
                        imageStream.CopyTo(fs);
                    }
                }
            }
        }

        public static IEnumerable<string> GetImages
        {
            get
            {
                return Directory.EnumerateFiles(ImagesDirectory);
            }
        }
    }
}
