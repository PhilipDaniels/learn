int main(int argc, char *argv[])
{
    // Lippman, $4.1, p135.
    // lvalue = an expression that yields an object or function. Can be const.
    // rvalue =

    // When we use an object as an lvalue we use the object's identity (its location in memory)
    // When we use an object as an rvalue we use the object's value (its contents)

    // We can use an lvalue when an rvalue is required, but not vice-versa.
    // EXCEPTION: (p531)
    // When we do this, we use the object's value.


    /*
      Operator               Requires                             Returns
      ===========================================================================================
      arithmetic ops         Requires: rvalue
      logical ops            Returns : rvalue
      relational ops

      = (assignment)         Requires: non-const lvalue as left operand
                             Returns : left operand as lvalue

      & (address of)         Requires: lvalue
                             Returns :  pointer to operand as an rvalue

      * (derefence)

      -> (member derefence)  Requires: a pointer operand
                             Returns : an lvalue

      [n] (subscript)

      *it (iterator deref)

      -- (decrement)         Requires: lvalue
      ++ (increment)         Returns : prefix version returns lvalue
                                       postfix returns a copy of the original value as an lvalue



      Given: int *p;
      Then: decltype(*p) yields int&         (because *p is an lvalue)
            decltype(&p) yields int**

     */
    return 0;
}
