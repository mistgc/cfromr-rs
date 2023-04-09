archive: clean compile
	ar rs libblurhash.a read_image.o blurhash_encode.o

compile: read_image.c blurhash_encode.c
	gcc -c $^


clean:
	-rm read_image.o blurhash_encode.o libblurhash.a


.PHNOY: compile clean
