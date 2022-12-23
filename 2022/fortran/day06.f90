module day06
  implicit none
  private
  character(len=1) :: a
  integer :: i,ierr
  integer(8) :: fish(0:8)=0,step(9,9)=0,mat(9,9)=0

  public :: solve06
contains

  ! This code is credit of u/autid, who inspired me to take another
  ! look at fortran
  subroutine solve06()
    open(1,file="../inputs/day06.txt")
    do
       read(1,'(i1,a1)',advance="no",iostat=ierr) i,a
       if(ierr.ne.0) exit
       fish(i) = fish(i)+1
    end do
    fish(i) = fish(i)+1
    close(1)

    forall(i=1:9) mat(i,i)=1
    step = cshift(mat, -1)
    step(1,7)=1

    do i=1,80
       mat=matmul(step,mat)
    end do
    write(*,'(a,i0)') "part 1: ", sum(matmul(fish,mat))

    do i=81,256
       mat=matmul(step,mat)
    end do
    write(*, '(a,i0)') "part 2: ", sum(matmul(fish,mat))

  end subroutine solve06

end module day06
