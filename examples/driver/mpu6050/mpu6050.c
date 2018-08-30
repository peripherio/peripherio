#include <string.h>
#include <stdlib.h>
#include <stdio.h>
#include <glob.h>
#include <unistd.h>
#include <stdint.h>
#include <fcntl.h>
#include <sys/ioctl.h>
#include <linux/i2c-dev.h>

#include "rami.gen.h"

#define ACCEL_RANGE_2G 0x00
#define ACCEL_RANGE_4G 0x08
#define ACCEL_RANGE_8G 0x10
#define ACCEL_RANGE_16G 0x18

static const float GRAVITY_MS2 = 9.80665;
static const float ACCEL_SCALE_MODIFIER_2G = 16384.0;
static const float ACCEL_SCALE_MODIFIER_4G = 8192.0;
static const float ACCEL_SCALE_MODIFIER_8G = 4096.0;
static const float ACCEL_SCALE_MODIFIER_16G = 2048.0;

static const float GYRO_SCALE_MODIFIER_250DEG = 131.0;
static const float GYRO_SCALE_MODIFIER_500DEG = 65.5;
static const float GYRO_SCALE_MODIFIER_1000DEG = 32.8;
static const float GYRO_SCALE_MODIFIER_2000DEG = 16.4;

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

int i2c_read(int fd, uint8_t reg, uint8_t* dat) {
  if ((write(fd, &reg, 1)) != 1) {
    return -1;
  }

  if (read(fd, dat, 1) != 1) {
    return -1;
  }
  return 0;
}

int i2c_read_word(int fd, uint8_t reg, uint16_t* dat) {
  uint8_t high;
  if (i2c_read(fd, reg, &high) != 0) {
    return -1;
  }

  uint8_t low;
  if (i2c_read(fd, reg+1, &low) != 0) {
    return -1;
  }

  uint16_t value = (high << 8) + low;

  if (value >= 0x8000) {
    value = -((0xFFFF - value) + 1);
  }

  *dat = value;
}

int use_config(Config* conf) {
  char dev[15];
  sprintf(dev, "/dev/i2c-%d", conf->if_i2c_busnum);
  int fd = open(dev, O_RDWR);
  if (fd < 0) {
    return -1;
  }
  if (ioctl(fd, I2C_SLAVE, conf->if_i2c_address) < 0) {
    return -1;
  }
  return fd;
}

int accel_range_to_mod(uint8_t raw_data) {
  switch(raw_data) {
    case ACCEL_RANGE_2G:
      return 2;
    case ACCEL_RANGE_4G:
      return 4;
    case ACCEL_RANGE_8G:
      return 8;
    case ACCEL_RANGE_16G:
      return 16;
    default:
      return -1;
  }
}

get_gyro_returns* get_gyro(get_gyro_args* args, Config* conf) {
  /* Your Implementation! */
}

get_accel_returns* get_accel(get_accel_args* args, Config* conf) {
  int fd = use_config(conf);
  if (fd < 0) {
    return NULL;
  }

  uint8_t x, y, z;
  i2c_read(fd, ACCEL_XOUT0, &x);
  i2c_read(fd, ACCEL_YOUT0, &y);
  i2c_read(fd, ACCEL_ZOUT0, &z);

  uint8_t accel_range;
  if(i2c_read(fd, ACCEL_CONFIG, &accel_range) != 0) {
    return NULL;
  }

  int mod = accel_range_to_mod(accel_range);
  if(mod < 0) {
    return NULL;
  }
  const double accel_scale_modifier = 32768.0 / mod;

  get_accel_returns* res = malloc(sizeof(get_accel_returns));
  res->x = x / accel_scale_modifier * GRAVITY_MS2;
  res->y = y / accel_scale_modifier * GRAVITY_MS2;
  res->z = z / accel_scale_modifier * GRAVITY_MS2;

  close(fd);
  return res;
}

get_temperature_returns* get_temperature(get_temperature_args* args, Config* conf) {
  int fd = use_config(conf);
  if (fd < 0) {
    return NULL;
  }

  uint16_t raw_temp;
  i2c_read_word(fd, TEMP_OUT0, &raw_temp);
  const double actual_temp = (raw_temp / 340.0) + 36.53;

  get_temperature_returns* res = malloc(sizeof(get_temperature_returns));
  res->value = actual_temp;

  close(fd);
  return res;
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
      close(fd);
      return NULL;
    }
    for (uint8_t addr = 0x03; addr < 0x78; addr++) {
      if (ioctl(fd, I2C_SLAVE, addr) < 0) {
        continue;
      }

      /* Read who_am_i */
      uint8_t dat;
      if (i2c_read(fd, 0x75, &dat) != 0) {
        close(fd);
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
    close(fd);
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

