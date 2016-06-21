CXX = g++
SRCS = $(wildcard *.cpp)
PROGS = $(patsubst %.cpp,%.exe,$(SRCS))
CFLAGS += --std=c++11 -ggdb -O0

all: $(PROGS)

%.exe: %.cpp
	$(CXX) $(CFLAGS) -o $@ $<

clean:
	rm -f $(PROGS) *.exe *.stackdump
