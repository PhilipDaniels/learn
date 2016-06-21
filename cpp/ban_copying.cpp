class Car
{
public:
    // To ban copying, simply delete the copy constructor and
    // the copy-assignment operator.
    Car(const Car&) = delete;
    Car& operator=(const Car&) = delete;
};

int main(int argc, char *argv[])
{
    return 0;
}
