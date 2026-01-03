CXX = g++
CXXFLAGS = -Wall -O2

SOURCES = $(wildcard cpp/*.cpp)
TARGETS = $(SOURCES:.cpp=)

.PHONY: all clean compile_rust FORCE

all: $(TARGETS)

%: %.cpp FORCE
	$(CXX) $(CXXFLAGS) $< -o $@

FORCE:

clean:
	rm -f $(TARGETS)
