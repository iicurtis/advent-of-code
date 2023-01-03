module aoc_2022_fortran
  implicit none
  private

  public :: say_hello
contains
  subroutine say_hello
    print *, "Hello, world!"
  end subroutine say_hello
end module aoc_2022_fortran
