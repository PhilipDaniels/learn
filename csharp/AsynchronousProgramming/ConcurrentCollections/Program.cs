﻿using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace ConcurrentCollections
{
    class Program
    {
        static void Main(string[] args)
        {
            ConcurrentDictionaryExample.Run();

            Console.WriteLine("Press any key...");
            Console.ReadKey();
        }
    }
}
