
int main(int argc, char *argv[])
{
    /*
                     ios_base            basic_streambuf<>
                         .               streambuf / wstreambuf
                         .
                     basic_ios<>
                     ios / wios
                     .        .
                   .            .
                (virtual)       (virtual)
                 .                .
    basic_istream<>             basic_ostream<>
    istream / wistream          ostream / wostream
                   .              .
                    .            .
                     .          .
                  basic_iostream<>
                 iostream / wiostream
     */

    /*
    ios_base mainly consists of components and functions for state and format flags.

    basic_ios defines the common properties of all stream classes, including the
    buffer used by the stream. It performs the actual reading and writing (actually
    it handles the *formatting*, an embedded streambuf manages the reading and
    writing.)

    basic_istream and basic_ostream define objects that can be used for reading
    and writing.

    basic_streambuf is the heart of the IO stream library. This class defines the
    interface to all representations that can be written to or read from by streams
    and is used by the other stream classes to perform the reading and writing of
    characters. For access to some external representation, classes are derived
    from basic_streambuf<>.

    Headers
    =======
    In hpp files, #include <iosfwd> if you can. This only includes class definitions
    though, there are no objects (so no cout etc.)

    For the objects, #include <istream> or <ostream>.
    #include <iostream> only if you are using the standard objects (cout etc.)

    TODO: Need to revisit my .emacs.pd.el.

    */

    return 0;
}
