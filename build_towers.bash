#!/usr/bin/env bash

gcc \
-o towers \
-lm \
$(pkg-config --libs -cflags gtk+-3.0) \
drawing.c \
dsf.c \
grid.c \
gtk.c \
latin.c \
malloc.c \
matching.c \
midend.c \
misc.c \
no-icon.c \
penrose.c \
printing.c \
ps.c \
puzzles.h \
random.c \
towers.c  \
tree234.c \
version.c \
&& ./towers
