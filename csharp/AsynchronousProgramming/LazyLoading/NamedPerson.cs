using HelperLib;

namespace LazyLoading
{
    public class NamedPerson
    {
        public readonly string Name;

        public NamedPerson(string name)
        {
            Name = name;
            Jobs.Delay(2000, "Creating a NamedPerson - " + Name);
        }
    }
}
