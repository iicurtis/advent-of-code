program advent

  use day06, only: solve06
  use iso_fortran_env
  implicit none
  integer(int64) :: c1,c2,cr,cm

  call system_clock(count_rate=cr)
  call system_clock(count_max=cm)
  write(*,*) "system_clock rate ", cr

  call system_clock(c1)
  call solve06()
  call system_clock(c2)

  write(*,*) "system_clock: ",(c2 - c1) / real(cr) * 1e6
end program advent
