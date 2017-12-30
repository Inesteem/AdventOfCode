
CC=.cpp
CXX=g++
RM=rm -f
CPPFLAGS=-g -std=c++14
LDFLAGS=-g -std=c++14

SRCS=main.cpp
OBJS=$(subst .cpp,.o,$(SRCS))

out: $(OBJS)
	$(CXX) $(LDFLAGS) -o out $(OBJS) $(LDLIBS) 


depend: .depend

.depend: $(SRCS) 
	$(RM) ./.depend
	$(CXX) $(CPPFLAGS) -MM $^>>./.depend;

clean:
	$(RM) $(OBJS)

distclean: clean
	$(RM) out

include .depend
