! Advent of Code Solutions
! Copyright (C) 2022  Isaac Curtis
program day03
  use iso_fortran_env
  use aoc_2022_fortran
  implicit none

  integer(int32) :: iunit,ierr,fid,nread
  character(:), allocatable    ::  line, first, second
  character(len=1024) :: buffer, ioerrmsg
  integer(int64) :: part1, part2, n_chars, i, elves, seen1, seen2, group_elves(3)
  call initialize_day
  
  part1 = 0
  part2 = 0
  elves = 1

  nread = 0
  open(newunit=fid,file="../inputs/day03.txt", status='old')
  do
    ! Very long custom read section, because fotran kind of sucks at i/o
    buffer = ''
    line = ''
    do
      read(fid,'(A)',advance='NO',size=nread,iostat=ierr,iomsg=ioerrmsg) buffer
      if (is_iostat_eor(ierr)) then
        ! add the last block of text before the end of record
        if (nread>0) line = line//buffer(1:nread)
        if (nread.eq.0) goto 20 ! if line is empty, stop reading
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

    seen1 = 0
    seen2 = 0
    n_chars = len(line)/2
    first = line(1:n_chars); second = line(n_chars+1:)
    do i = 1, n_chars
      seen1 = ibset(seen1, value_branchless(first(i:i)))
      seen2 = ibset(seen2, value_branchless(second(i:i)))
    end do
    part1 = part1 + trailz(iand(seen1, seen2))
    group_elves(elves) = ior(seen1, seen2)
    if (elves == 3) then
      part2 = part2 + trailz(iand(group_elves(1), iand(group_elves(2), group_elves(3))))
    end if
    elves = merge(1_int64, elves+1, elves==3)
  end do
  20 close(fid)

  write(*,*) "Running Day03"
  write(*,"(a,i10)") "  part 1: ", part1
  write(*,"(a,i10)") "  part 2: ", part2
  call time_day

  contains
  integer function value(c1)
    character(len=1), intent(in) :: c1
    if(ichar(c1) >= ichar('a')) then
      value = 1 + ichar(c1) - ichar('a')
    else
      value = 26 + 1 + ichar(c1) - ichar('A')
    end if
  end function value

  integer function value_branchless(c1)
    character(len=1), intent(in) :: c1
    integer :: cint, c
    value_branchless = mod(ichar(c1) - 38, 58)
  end function value_branchless

end program
