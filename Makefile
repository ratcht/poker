CC = g++
CFLAGS = -Wall -g -Iinclude -pthread
LD = g++
LDFLAGS = -g -pthread
LDLIBS = -lz -lcurl  # link with libz

# Directory structure
SRCDIR = src
INCDIR = include
OBJDIR = obj
OUTDIR = bin

# Source files with main functions
MAIN_SRCS = $(SRCDIR)/poker.cpp
MAIN_OBJS = $(patsubst $(SRCDIR)/%.cpp, $(OBJDIR)/%.o, $(MAIN_SRCS))

# Other (shared) source files - need to handle subdirectories properly
COMMON_SRCS = $(filter-out $(MAIN_SRCS), $(wildcard $(SRCDIR)/*.cpp) $(wildcard $(SRCDIR)/*/*.cpp))
COMMON_OBJS = $(patsubst $(SRCDIR)/%.cpp, $(OBJDIR)/%.o, $(COMMON_SRCS))

# Final executable names
TARGETS = $(OUTDIR)/poker.out

# Create obj directory structure
$(shell mkdir -p $(OBJDIR))
$(shell mkdir -p $(OUTDIR))

# Default target
all: $(TARGETS)

# Executable targets
$(TARGETS): $(OBJDIR)/poker.o $(COMMON_OBJS)
	$(LD) $(LDFLAGS) -o $@ $^ $(LDLIBS)

# Compile rule for object files
$(OBJDIR)/%.o: $(SRCDIR)/%.cpp
	@mkdir -p $(dir $@)
	$(CC) $(CFLAGS) -c $< -o $@

# Dependency files
DEPS = $(COMMON_OBJS:.o=.d) $(MAIN_OBJS:.o=.d)

$(OBJDIR)/%.d: $(SRCDIR)/%.c
	@mkdir -p $(dir $@)
	@$(CC) -MM -MT $(@:.d=.o) -MF $@ $< -I$(INCDIR)

-include $(DEPS)

# Debug targets to check variables
debug:
	@echo "MAIN_SRCS: $(MAIN_SRCS)"
	@echo "COMMON_SRCS: $(COMMON_SRCS)"
	@echo "MAIN_OBJS: $(MAIN_OBJS)"
	@echo "COMMON_OBJS: $(COMMON_OBJS)"

.PHONY: all clean debug

clean:
	rm -rf $(OBJDIR) $(TARGETS) $(OUTDIR)
