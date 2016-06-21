int main(int argc, char *argv[])
{
    /*
    [const] TYPE * [const] VARIABLE

    VARIABLE is used to point to data of type TYPE through *VARIABLE

    Draw a line through the * or multiple *s

    If there is a const to the left of the * it applies to the data and the data
    cannot be changed: *VARIABLE cannot be assigned, except at initialization.
    This is called a "top level const".

    If there is a const to the right of the * it applies to the VARIABLE and
    what the VARIABLE points to cannot be changed: VARIABLE cannot be assigned,
    except at initialization.
    */

    int       *       i1;      // can change    *i1     can change i1
    int const *       i2;      // cannot change *i2     can change i2
    int       * const i3 = 0;  // can change    *i3  cannot change i3
    int const * const i4 = 0;  // cannot change *i4  cannot change i4

    /*
     Confusingly, "top level consts" can also be written like this, putting
     the const before the type name.
    */
    const int *       i5;
    const int * const i6 = 0;

    // http://c-faq.com/decl/spiral.anderson.html
    // Also some good examples here: http://stackoverflow.com/questions/1143262

    return 0;
}
