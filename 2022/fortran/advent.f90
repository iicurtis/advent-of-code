program advent

  use day01, only: solve01
  use iso_fortran_env
  implicit none
  integer(int64) :: c1,c2,cr,cm

  call system_clock(count_rate=cr)
  call system_clock(count_max=cm)
  write(*,*) "system_clock rate ", cr

  call system_clock(c1)
  call solve01()
  call system_clock(c2)
  write(*,*) "day01 system_clock: ",(c2 - c1) / real(cr) * 1e3, "ms"

end program advent
