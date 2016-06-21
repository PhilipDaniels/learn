#ifndef UNIQUE_RESOURCE_HPP
#define UNIQUE_RESOURCE_HPP

/*
// An implementation of http://www.open-std.org/jtc1/sc22/wg21/docs/papers/2014/n4189.pdf
// g++ will compile this but only in c++14 mode.
// I aldo had to change the noexcept specifications to remove use of "this",
// add also re-order some of the functions.


// $$ 4 Motivation and scope:
// While std::unique_ptr can be tweaked by using a custom deleter type to almost
// a perfect handler for resources, it is awkward to use for handle types that
// are not pointers and for the use case of a scope guard. As a smart pointer
// std::unique_ptr can be used syntactically like a pointer, but requires the
// use of get() to pass the underlying pointer value to legacy APIs.
// This proposal introduces a new RAII "smart" resource container called
// unique_resource which can bind a resource to "clean-up" code regardless of
// type of the argument required by the "clean-up" function.


// $$ 4.4
// The expected method of constuction is to use one of
//
// unique_resource(resources,deleter) - non-checking instance, allows
// multiple parameters.
//
// unique_resource_checked(resource, invalid_value,deleter) - checked  instance,
// allowing a resource which is validated to inhibit the call to the deleter
// function if invalid.


// $$ 4.5 This class does not do reference counting (i.e. it is a resource
// version of unique_ptr, not shared_ptr).


namespace std {
namespace experimental {

template<typename R,typename D>
class unique_resource
{
  R resource;
  D deleter;
  bool execute_on_destruction;
  // exposition only
  unique_resource& operator=(unique_resource const &) = delete;
  unique_resource(unique_resource const &) = delete; // no copies

public:
  // construction
  explicit unique_resource(R && resource, D && deleter, bool shouldrun = true) noexcept
    : resource(std::move(resource)),
      deleter(std::move(deleter)),
      execute_on_destruction { shouldrun } { }

  // move
  unique_resource(unique_resource &&other) noexcept
    : resource(std::move(other.resource)),
      deleter(std::move(other.deleter)),
      execute_on_destruction{other.execute_on_destruction}
  {
    other.release();
  }

  // deleter access
  const D & get_deleter() const noexcept
  {
    return this->deleter;
  }

  void reset() noexcept(noexcept(get_deleter()(resource)))
  {
    if (execute_on_destruction)
    {
      this->execute_on_destruction = false;
      this->get_deleter()(resource);
    }
  }

  // unique_resource& operator=(unique_resource &&other) noexcept(noexcept(this->reset()))
  unique_resource& operator=(unique_resource &&other) noexcept(noexcept(reset()))
  {
    this->reset();
    this->deleter=std::move(other.deleter);
    this->resource=std::move(other.resource);
    this->execute_on_destruction=other.execute_on_destruction;
    other.release();
    return *this;
  }

  // resource release
  // ~unique_resource() noexcept(noexcept(this->reset()))
  ~unique_resource() noexcept(noexcept(reset()))
  {
    this->reset();
  }

  // void reset(R && newresource) noexcept(noexcept(this->reset()))
  void reset(R && newresource) noexcept(noexcept(reset()))
  {
    this->reset();
    this->resource = std::move(newresource);
    this->execute_on_destruction = true;
  }

  R const & release() noexcept
  {
    this->execute_on_destruction = false;
    return this->get();
  }

  // resource access
  R const & get() const noexcept
  {
    return this->resource;
  }

  operator R const &() const noexcept
  {
    return this->resource;
  }

  R operator->() const noexcept
  {
    return this->resource;
  }

  std::add_lvalue_reference_t<std::remove_pointer_t<R>>
  operator*() const
  {
    return * this->resource;
  }
};

// Factories
template<typename R, typename D>
auto make_unique_resource(R && r, D &&d) noexcept
{
  return unique_resource<R, std::remove_reference_t<D>>(
    std::move(r),
    std::forward<std::remove_reference_t<D>>(d),
    true);
}

template<typename R, typename D>
auto make_unique_resource_checked(R r, R invalid, D d) noexcept
{
  bool shouldrun = not bool(r == invalid);
  return unique_resource<R,D>(std::move(r), std::move(d), shouldrun);
}


}
}

*/

#endif /* UNIQUE_RESOURCE_HPP */
