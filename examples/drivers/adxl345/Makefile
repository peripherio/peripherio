
driver.so: adxl345.c peripherio.gen.h
	$(CC) adxl345.c -shared -o driver.so

peripherio.gen.h: driver.yml
	PERIPHERIO_DRIVER_PATH=$(realpath ../) \
	PERIPHERIO_CATEGORY_PATH=$(realpath ../../categories) \
	../../../target/debug/codegen
