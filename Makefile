CXX = g++
CXXFLAGS = -Wall -O2

# Find all .cpp files in the cpp directory
SOURCES = $(wildcard cpp/*.cpp)

# Determine the target executable names (remove .cpp extension)
TARGETS = $(SOURCES:.cpp=)

.PHONY: all clean compile_rust FORCE

all: $(TARGETS)

# Rule to compile each .cpp file into its own executable
# This pattern rule matches a target like 'cpp/read_sync' to 'cpp/read_sync.cpp'
%: %.cpp FORCE
	$(CXX) $(CXXFLAGS) $< -o $@

FORCE:

clean:
	rm -f $(TARGETS)
