! Advent of Code Solutions
! Copyright (C) 2023  Isaac Curtis
program day01
  use iso_fortran_env
  use aoc_2023_fortran
  use stdlib_string_type
  use stdlib_stringlist_type
  use stdlib_strings, only: replace_all
  use stdlib_ascii, only: is_digit
  implicit none

  integer :: fid, iunit, ierr, nread, i
  type(string_type) :: line, digits
  character(len=1024) :: ioerrmsg, buffer
  character :: c
  integer(int64) :: first_last, part1, part2
  type(stringlist_type) :: num_strings, repl_strings
  call initialize_day



  num_strings = stringlist_type([ &
    'one', &
    'two', &
    'three', &
    'four', &
    'five', &
    'six', &
    'seven', &
    'eight', &
    'nine'])

  repl_strings = stringlist_type([ &
    'o1e', &
    't2o', &
    't3e', &
    'f4r', &
    'f5e', &
    's6x', &
    's7n', &
    'e8t', &
    'n9e'])

  part1 = 0
  open(newunit=fid, file="../inputs/day01.txt", status="old")
  do
    ! Very long custom read section, because fotran kind of sucks at i/o
    buffer = ''
    line = ''
    digits = ''
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

    write(*,*) line
    do i = 1, len(line)
      c = char(line, i)
      if (is_digit(c)) digits = digits//c
    end do
    first_last = to_int(char(digits, 1))*10 + to_int(char(digits, len(digits)))
    part1 = part1 + first_last

    do i = 1, size(num_strings)
      line = replace_all(line, num_strings%get(fidx(i)), repl_strings%get(fidx(i)))
    end do
    write(*,*) " ", first_last
  end do
  20 close(fid)

  write(*,*) "Running Day01"
  write(*,"(a,i10)") "  part 1: ", part1
  write(*,"(a,i10)") "  part 2: ", part2
  call time_day

  contains
  integer function to_int(c1)
    character(len=1), intent(in) :: c1
    to_int = ichar(c1) - ichar('0')
  end function to_int
end program day01
