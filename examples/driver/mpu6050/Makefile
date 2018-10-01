
driver.so: mpu6050.c peripherio.gen.h
	$(CC) mpu6050.c -shared -o driver.so

peripherio.gen.h: driver.yml
	PERIPHERIO_DRIVER_PATH=$(realpath ../) \
	PERIPHERIO_CATEGORY_PATH=$(realpath ../../category) \
	../../../target/debug/codegen
