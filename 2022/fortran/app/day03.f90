! Advent of Code Solutions
! Copyright (C) 2022  Isaac Curtis
program day03
  use iso_fortran_env
  implicit none

  integer(int32) :: iunit,ierr,fid,nread
  character(:), allocatable    ::  line, first, second
  integer(int64) :: part1, part2, n_chars, i, elves
  logical  :: bits1(128)=.false.,bits2(128)=.false.
  character(len=1) :: c
  character(len=1024) :: buffer, ioerrmsg, group_elves(3)
  integer(int64) :: time_start,time_end,cr,cm
  call system_clock(count_rate=cr)
  call system_clock(count_rate=cm)
  call system_clock(time_start)

  part1 = 0
  part2 = 0
  elves = 1

  nread = 0
  open(newunit=fid,file="../inputs/day03.txt", status='old')
  do
     buffer = ''
     line = ''
     do
     read(fid,'(A)',advance='NO',size=nread,iostat=ierr,iomsg=ioerrmsg) buffer
     if (is_iostat_eor(ierr)) then
        ! add the last block of text before the end of record
        if (nread>0) line = line//buffer(1:nread)
        exit
     else if (is_iostat_end(ierr)) then
        goto 20
     else if (ierr==0) then ! all the characters were read
        line = line//buffer  ! add this block of text to the string
     else  ! some kind of error
        write(*,*) ierr, ioerrmsg
        error stop 'Read error'
        exit
     end if
     end do

     n_chars = len(line)/2
     first = line(1:n_chars); second = line(n_chars+1:)
     do i = 1, n_chars
        c = first(i:i) ! check character of first split in second
        if (index(second, c)>0) then
           part1 = part1 + value(c)
           exit
        end if
     end do
     group_elves(elves) = line
     if (elves == 3) then
        do i = 1, len(line)
           if(index(group_elves(1), line(i:i)) > 0 .and. index(group_elves(2), line(i:i))>0) then
              part2 = part2 + value(line(i:i))
              exit
           end if
        end do
     end if
     elves = merge(1_int64, elves+1, elves==3)
  end do
20 close(fid)

  call system_clock(time_end)
  write(*,*) "Running Day03"
  write(*,"(a,i10)") "  part 1: ", part1
  write(*,"(a,i10)") "  part 2: ", part2
  write(*,"(a,f12.6,a)") "  system_clock: ", (time_end - time_start) / real(cr) * 1e6, " μs"

  contains
  integer function value(c1)
    character(len=1), intent(in) :: c1
    integer :: j
    if(ichar(c1) >= ichar('a')) then
       value = 1 + ichar(c1) - ichar('a')
    else
       value = 26 + 1 + ichar(c1) - ichar('A')
    end if
  end function

end program day03
