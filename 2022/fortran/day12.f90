module day12
    implicit none
    integer :: i,j,k,n,m,ierr,start,end
    character(len=5) :: name,a,b
    character(len=5), allocatable :: cavenames(:)
    character(len=8), allocatable :: connections(:)
    logical, allocatable :: visited(:)
    integer, allocatable :: conn(:,:)
    public :: solve12
contains
    subroutine solve12()
        open(1,file="../inputs/day12.txt")
        n=0
        do
            read(1,*,iostat=ierr)
            if(ierr.ne.0) exit
            n=n+1
        end do
        rewind(1)
        allocate(connections(n),cavenames(2*n),conn(2*n,2*n),visited(2*n))
        read (1,*) connections
        close(1)

        cavenames=""
        cavenames(1) = "start"
        conn=0
        m=2
        do i=1,n
            name = connections(i)(1:scan(connections(i),"-")-1)
            if((.not.any(cavenames.eq.name)).and.(trim(name).ne."end")) then
                cavenames(m) = name
                m=m+1
            end if
            name = connections(i)(scan(connections(i),"-")+1:len_trim(connections(i)))
            if((.not.any(cavenames.eq.name)).and.(trim(name).ne."end")) then
                cavenames(m) = name
                m=m+1
            end if
        end do
        cavenames(m)="end"
        do i=1,n
            a = connections(i)(1:scan(connections(i),"-")-1)
            b = connections(i)(scan(connections(i),"-")+1:len_trim(connections(i)))
            j=maxloc(cavenames,mask=cavenames.eq.a,dim=1)
            k=maxloc(cavenames,mask=cavenames.eq.b,dim=1)
            conn(minloc(conn(:,j)),j) = k
            conn(minloc(conn(:,k)),k) = j 
        end do
        visited = .false.
        write(*,'(a,i0)') "part 1: ", npaths(conn,visited,1,m,1,.false.)
        write(*,'(a,i0)') "part 2: ", npaths(conn,visited,1,m,2,.false.)
    end subroutine solve12

    recursive function npaths(conn,v,a,end,part,double) result(p1)
        integer :: conn(:,:),i,j,a,end,p1,part
        logical :: v(:),visited(size(visited)),double

        visited = v
        visited(a)=.true.
        p1=0
        do i=1,size(conn,dim=1)
            if(conn(i,a).eq.1)cycle
            if(conn(i,a).eq.0)exit
            if(conn(i,a).eq.end) then
                p1=p1+1
                cycle
            else
                j=iachar(cavenames(conn(i,a))(1:1))
                if(j.ge.97 .and. j.le.122) then
                    if(visited(conn(i,a))) then
                        if(double .or. part.eq.1) cycle
                        p1=p1+npaths(conn,visited,conn(i,a),end,part,.true.)
                        cycle
                    endif
                end if
                p1=p1+npaths(conn,visited,conn(i,a),end,part,double)
            end if
        end do
    end function npaths
end module day12
