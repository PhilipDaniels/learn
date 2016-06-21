using HelperLib;

namespace LazyLoading
{
    public class Person
    {
        public Person()
        {
            Jobs.Delay(2000, "Creating a Person");
        }
    }
}
