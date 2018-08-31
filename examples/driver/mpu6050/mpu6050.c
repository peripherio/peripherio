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

#define RANGE_2G 0x00
#define RANGE_4G 0x08
#define RANGE_8G 0x10
#define RANGE_16G 0x18

static const float GRAVITY_MS2 = 9.80665;

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

int i2c_write(int fd, uint8_t reg, uint8_t dat) {
  uint8_t buf[2] = {reg, dat};
  if (write(fd, buf, 2) != 2) {
    return -1;
  }
  return 0;
}

int i2c_read_word(int fd, uint8_t reg, int16_t* dat) {
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
  if (i2c_write(fd, PWR_MGMT_1, 0x00) != 0) {
    return -1;
  }
  return fd;
}

int range_to_mod(uint8_t raw_data) {
  switch(raw_data) {
    case RANGE_2G:
      return 2;
    case RANGE_4G:
      return 4;
    case RANGE_8G:
      return 8;
    case RANGE_16G:
      return 16;
    default:
      return -1;
  }
}

get_gyro_returns* get_gyro(get_gyro_args* args, Config* conf) {
  int fd = use_config(conf);
  if (fd < 0) {
    return NULL;
  }

  int16_t x, y, z;
  i2c_read_word(fd, GYRO_XOUT0, &x);
  i2c_read_word(fd, GYRO_YOUT0, &y);
  i2c_read_word(fd, GYRO_ZOUT0, &z);

  uint8_t gyro_range;
  if(i2c_read(fd, GYRO_CONFIG, &gyro_range) != 0) {
    return NULL;
  }

  int mod = range_to_mod(gyro_range);
  if(mod < 0) {
    return NULL;
  }
  const double gyro_scale_modifier = 262.0 / mod;

  get_gyro_returns* res = malloc(sizeof(get_gyro_returns));
  res->x = x / gyro_scale_modifier;
  res->y = y / gyro_scale_modifier;
  res->z = z / gyro_scale_modifier;

  close(fd);
  return res;
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

  int mod = range_to_mod(accel_range);
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

  int16_t raw_temp;
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
    const char* path = globbuf.gl_pathv[i];
    int fd = open(path, O_RDWR);
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
        continue;
      }
      if(dat != 0b01101000) {
        continue;
      }

      /* Device found with bus/addr */
      Config* new_config = malloc(sizeof(Config));
      memcpy(new_config, conf, sizeof(Config));

      new_config->if_i2c_speed = (int64_t)100;
      new_config->if_i2c_address = addr;

      /* parse glob result and obtain bus number */
      unsigned ci = 0;
      for (ci = strlen(path); ci > 0; ci--) {
        if (path[ci] == '-') { // 3 in i2c-n
          break;
        }
      }
      long busnum = strtol(path + ci + 1, NULL, 10);
      new_config->if_i2c_busnum = busnum;


      results[results_count++] = new_config;
    }
    close(fd);
  }
  globfree(&globbuf);

  *size = results_count;
  Config** res = malloc(sizeof(Config*)*results_count);
  for(size_t i = 0; i < results_count; i++) {
    res[i] = malloc(sizeof(Config));
    *res[i] = *results[i];
  }
  return res;
}

