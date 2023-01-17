! Advent of Code Solutions
! Copyright (C) 2022  Isaac Curtis
program day04
  use iso_fortran_env
  use aoc_2022_fortran
  implicit none

  integer(int32) :: iunit,ierr,fid,nread
  character(len=1024) :: buffer, ioerrmsg
  integer(int64) :: part1, part2, elf1(2), elf2(2)
  call initialize_day
  
  part1 = 0
  part2 = 0

  open(newunit=fid,file="../inputs/day04.txt", status='old')
  do
    read(fid, "(a)", end=20, iostat=ierr) buffer
    if (len_trim(buffer) == 0) exit
    do iunit = 1, len_trim(buffer)
      if (buffer(iunit:iunit) == "-") buffer(iunit:iunit) = " "
    end do
    read(buffer, *) elf1, elf2
    if ( product(elf1 - elf2) <= 0 ) then ! either one negative or matched start/end
      part1 = part1 + 1
    end if
    if (.not. (elf2(1) > elf1(2) .or. elf1(1) > elf2(2))) then
      part2 = part2 + 1
    end if
  end do
  20 close(fid)

  write(*,*) "Running Day04"
  write(*,"(a,i10)") "  part 1: ", part1
  write(*,"(a,i10)") "  part 2: ", part2
  call time_day

end program
