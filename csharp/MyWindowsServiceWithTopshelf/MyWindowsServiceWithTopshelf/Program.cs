using Topshelf;

namespace MyWindowsServiceWithTopshelf
{
    class Program
    {
        static void Main(string[] args)
        {
            // See https://topshelf.readthedocs.io/en/latest/overview/commandline.html for details,
            // but basically:
            //     prog.exe install | start | stop | uninstall

            HostFactory.Run(hostConfigurator =>
            {
                hostConfigurator.Service<MyService>(serviceConfigurator =>
                {
                    serviceConfigurator.ConstructUsing(() => new MyService());
                    serviceConfigurator.WhenStarted(myService => myService.Start());
                    serviceConfigurator.WhenStopped(myService => myService.Stop());
                });

                hostConfigurator.RunAsLocalSystem();

                hostConfigurator.StartAutomaticallyDelayed();

                // The string you need to pass is the one shown at the top of the Service Properties
                // popup in Computer Management -> Services.
                hostConfigurator.DependsOn("PlugPlay");

                // Used to refer to the service by name in the SCM API, e.g. in DependsOn() above.
                hostConfigurator.SetServiceName("MyWindowsServiceName");

                // Appears as the leftmost column headed "Name" in Computer Management -> Services.
                hostConfigurator.SetDisplayName("MyWindowsService Display Name");

                // Appears as the column headed "Description" in Computer Management -> Services.
                hostConfigurator.SetDescription("MyWindowsService using Topshelf");
            });
        }
    }
}
