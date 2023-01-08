! Advent of Code Solutions
! Copyright (C) 2022  Isaac Curtis
program day02
  use iso_fortran_env
  implicit none

  integer(int32) :: iunit,ierr,fid
  integer(int64) :: part1, part2, abc, xyz, total_outcomes
  character    ::   a, b
  integer(int64) :: time_start,time_end,cr,cm
  call system_clock(count_rate=cr)
  call system_clock(count_rate=cm)
  call system_clock(time_start)

  part1 = 0
  part2 = 0
  total_outcomes = 3

  open(newunit=fid,file="../inputs/day02.txt", status='old')
  do
     read(fid,'(a1,1x,a1)',iostat=ierr) a,b
     if(ierr.ne.0) exit
     abc = ichar(a) - ichar('A')
     xyz = ichar(b) - ichar('X')
     select case ( mod(xyz + 3 - abc, 3_int64))
        case (0)
           part1 = part1 + xyz + 1 + 3
        case (1)
           part1 = part1 + xyz + 1 + 6
        case (2)
           part1 = part1 + xyz + 1
     end select
     select case (xyz)
        case (0)
           part2 = part2 + mod(abc + 2, 3_int64) + 1
        case (1)
           part2 = part2 + abc + 1 + 3
        case (2)
           part2 = part2 + mod(abc + 1, 3_int64) + 1 + 6
     end select
  end do
  close(fid)

  call system_clock(time_end)
  write(*,*) "Running Day02"
  write(*,"(a,i10)") "  part 1: ", part1
  write(*,"(a,i10)") "  part 2: ", part2
  write(*,"(a,f12.6,a)") "  system_clock: ", (time_end - time_start) / real(cr) * 1e6, " μs"

end program day02
