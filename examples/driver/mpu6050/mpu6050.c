#include <string.h>
#include <stdlib.h>
#include <glob.h>
#include <unistd.h>
#include <stdint.h>
#include <fcntl.h>
#include <sys/ioctl.h>
#include <linux/i2c-dev.h>

#include "rami.gen.h"

static const float GRAVITY_MS2 = 9.80665;
static const float ACCEL_SCALE_MODIFIER_2G = 16384.0;
static const float ACCEL_SCALE_MODIFIER_4G = 8192.0;
static const float ACCEL_SCALE_MODIFIER_8G = 4096.0;
static const float ACCEL_SCALE_MODIFIER_16G = 2048.0;

static const float GYRO_SCALE_MODIFIER_250DEG = 131.0;
static const float GYRO_SCALE_MODIFIER_500DEG = 65.5;
static const float GYRO_SCALE_MODIFIER_1000DEG = 32.8;
static const float GYRO_SCALE_MODIFIER_2000DEG = 16.4;

static const uint8_t ACCEL_RANGE_2G = 0x00;
static const uint8_t ACCEL_RANGE_4G = 0x08;
static const uint8_t ACCEL_RANGE_8G = 0x10;
static const uint8_t ACCEL_RANGE_16G = 0x18;

static const uint8_t GYRO_RANGE_250DEG = 0x00;
static const uint8_t GYRO_RANGE_500DEG = 0x08;
static const uint8_t GYRO_RANGE_1000DEG = 0x10;
static const uint8_t GYRO_RANGE_2000DEG = 0x18;

static const uint8_t PWR_MGMT_1 = 0x6B;
static const uint8_t PWR_MGMT_2 = 0x6C;

static const uint8_t ACCEL_XOUT0 = 0x3B;
static const uint8_t ACCEL_YOUT0 = 0x3D;
static const uint8_t ACCEL_ZOUT0 = 0x3F;

static const uint8_t TEMP_OUT0 = 0x41;

static const uint8_t GYRO_XOUT0 = 0x43;
static const uint8_t GYRO_YOUT0 = 0x45;
static const uint8_t GYRO_ZOUT0 = 0x47;

static const uint8_t ACCEL_CONFIG = 0x1C;
static const uint8_t GYRO_CONFIG = 0x1B;

get_gyro_returns* get_gyro(get_gyro_args* args, Config* conf) {
  /* Your Implementation! */
}

get_accel_returns* get_accel(get_accel_args* args, Config* conf) {
  /* Your Implementation! */
}

get_temperature_returns* get_temperature(get_temperature_args* args, Config* conf) {
  /* Your Implementation! */
}

void init() {
}

Config** detect(Config* conf, size_t* size) {
  Config* results[100];
  size_t results_count = 0;

  glob_t globbuf;
  glob("/dev/i2c-*", 0, NULL, &globbuf);
  for (int i = 0; i < globbuf.gl_pathc; i++) {
    int fd = open(globbuf.gl_pathv[i], O_RDWR);
    if (fd < 0) {
      return NULL;
    }
    for (uint8_t addr = 0x03; addr < 0x78; addr++) {
      if (ioctl(fd, I2C_SLAVE, addr) < 0) {
        continue;
      }

      /* Read who_am_i */
      uint8_t reg = 0x75;
      if ((write(fd, &reg, 1)) != 1) {
        return NULL;
      }
      uint8_t dat;
      if (read(fd, &dat, 1) != 1) {
        return NULL;
      }
      if(dat != 0b01101000) {
        continue;
      }

      /* Device found with bus/addr */
      Config* new_config = malloc(sizeof(Config));
      memcpy(new_config, conf, sizeof(Config));

      new_config->if_i2c_speed = 100l;
      new_config->if_i2c_busnum = i; // FIXME: Maybe have to parse the glob result
      new_config->if_i2c_address = addr;

      results[results_count++] = new_config;
      results_count++;
    }
  }
  globfree(&globbuf);

  *size = results_count;
  Config** res = malloc(sizeof(Config*)*results_count);
  for(size_t i = 0; i < results_count; i++) {
    res[i] = malloc(sizeof(Config));
    memcpy(res[i], results[i], sizeof(Config));
  }
  return res;
}

