module aoc_2022_fortran
  use iso_fortran_env
  implicit none
  private
  integer(int64) :: cr, cm, time_start, time_end

  public :: initialize_day, time_day
contains
  subroutine initialize_day
    call system_clock(count_rate=cr)
    call system_clock(count_max=cm)
    call system_clock(time_start)
  end subroutine
  subroutine time_day
    call system_clock(time_end)
    write(*,"(a,f12.6,a)") "  system_clock: ", (time_end - time_start) / real(cr) * 1e6, " μs"
  end subroutine
end module aoc_2022_fortran
