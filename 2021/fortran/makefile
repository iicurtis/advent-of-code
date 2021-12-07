##
# advent-of-code-2021
#
# @file
# @version 0.1

FC = gfortran
FCFLAGS = -g -O3 -w -Wall

OBJS = day06.o

.PHONY: all clean
.SUFFIXES: .f90 .o

all: advent

advent: advent.f90 $(OBJS)
	$(FC) $(FCFLAGS) $< $(OBJS) -o $@

.f90.o:
	$(FC) -c $(FCFLAGS) $<

%.o: %.mod

clean:
	$(RM) advent *.mod *.o

# end
