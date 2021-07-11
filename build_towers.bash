#!/usr/bin/env bash

(cd rust_towers; cargo build)
(cd rust_towers; cbindgen --config cbindgen.toml --crate rust_towers --output my_header.h)

gcc \
-o towers \
./rust_towers/my_header.h \
-lm \
$(pkg-config --libs -cflags gtk+-3.0) \
drawing.c \
dsf.c \
grid.c \
gtk.c \
latin.c \
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
-L rust_towers/target/debug/ \
-l rust_towers \
&& LD_LIBRARY_PATH=./rust_towers/target/debug ./towers
