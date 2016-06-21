struct Planet
{
    double radius;
};

class Satellite
{
    double radius;
};

int main(int argc, char *argv[])
{
    // The ONLY difference between class and struct is that in a struct
    // everything is public by default, and in a class everything is private
    // by default.
    Planet earth;
    earth.radius = 12;

    Satellite theMoon;
    //theMoon.radius = 20;

    // Guidance: * Only use structs for DTOs with no logic. Everything else
    // is a class.
}
