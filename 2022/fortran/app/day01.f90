! Advent of Code Solutions
! Copyright (C) 2022  Isaac Curtis
program day01
  use iso_fortran_env
  implicit none

  integer :: top(0:3)=0
  integer(int64) :: iunit,ierr,cals,cal_sum,fid
  integer(int64) :: time_start,time_end,cr,cm
  call system_clock(count_rate=cr)
  call system_clock(count_rate=cm)
  call system_clock(time_start)

  cal_sum = 0

  open(newunit=fid,file="../inputs/day01.txt", status='old')
  do
     read(fid,'(i10)',iostat=ierr) cals
     if(ierr.ne.0) exit
     if(cals.gt.0) then
        cal_sum = cal_sum + cals
     else
        if (cal_sum.gt.top(1)) then
           top = cshift(top, 1)
           top(1) = cal_sum
        else if (cal_sum.gt.top(2)) then
           top(3) = top(2)
           top(2) = cal_sum
        else if (cal_sum.gt.top(3)) then
           top(3) = cal_sum
        end if
        cal_sum = 0
     end if
  end do
  close(1)

  call system_clock(time_end)
  write(*,*) "Running Day01"
  write(*,"(a,i10)") "  part 1: ", top(1)
  write(*,"(a,i10)") "  part 2: ", sum(top)
  write(*,"(a,f12.6,a)") "  system_clock: ", (time_end - time_start) / real(cr) * 1e6, " μs"

end program day01
