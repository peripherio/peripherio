
driver.so: adxl345.c peripherio.gen.h
	$(CC) adxl345.c -shared -o driver.so

peripherio.gen.h: driver.yml
	PERIPHERIO_DRIVER=$(realpath ../) \
	PERIPHERIO_CATEGORY=$(realpath ../../category) \
	../../../target/debug/codegen
