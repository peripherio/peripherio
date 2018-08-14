#include <unistd.h>
#include <utility>
#include <bitset>

#include "i2c.hpp"

namespace rami::driver {

class mpu6050 {
public:
  static std::uint8_t const default_address = 0x68;

  static std::uint8_t detect(std::uint8_t bus) {
    return interface::i2c::find(bus, [](interface::i2c&& i2c) {
        auto const who_am_i = i2c.read(0x75);
        return who_am_i == 0b01101000;
    });
  }

  mpu6050(std::uint8_t bus, std::uint8_t address) : interface(interface::i2c(bus, address)) {
    this->interface.write(this->PWR_MGMT_1, 0x00);
  }

  std::uint16_t get_temp() {
    auto const raw_temp = this->read_word(this->TEMP_OUT0);
    auto const actual_temp = (raw_temp / 340.0) + 36.53;
    return actual_temp;
  }

  std::tuple<double, double, double> get_accel_data() {
    auto x = this->interface.read(this->ACCEL_XOUT0);
    auto y = this->interface.read(this->ACCEL_YOUT0);
    auto z = this->interface.read(this->ACCEL_ZOUT0);

    auto const accel_range = this->read_accel_range();

    if(accel_range == -1)
      return std::make_tuple(-1, -1, -1);

    auto const accel_scale_modifier = 32768.0 / accel_range;

    x /= accel_scale_modifier;
    y /= accel_scale_modifier;
    z /= accel_scale_modifier;

    x *= this->GRAVITY_MS2;
    y *= this->GRAVITY_MS2;
    z *= this->GRAVITY_MS2;
    return std::make_tuple(x, y, z);
  }

private:

  std::uint16_t read_word(std::uint8_t reg) {
    auto const high = this->interface.read(reg);
    auto const low = this->interface.read(reg+1);

    auto const value = (high << 8) + low;

    if (value >= 0x8000)
      return -((0xFFFF - value) + 1);
    else
      return value;
  }

  void set_accel_range(std::uint8_t accel_range) {
    this->interface.write(this->ACCEL_CONFIG, 0x00);
    this->interface.write(this->ACCEL_CONFIG, accel_range);
  }

  std::uint8_t read_accel_range(bool raw=false) {
    auto const raw_data = this->interface.read(this->ACCEL_CONFIG);

    if(raw)
      return raw_data;


    switch(raw_data) {
      case this->ACCEL_RANGE_2G:
        return 2;
      case this->ACCEL_RANGE_4G:
        return 4;
      case this->ACCEL_RANGE_8G:
        return 8;
      case this->ACCEL_RANGE_16G:
        return 16;
      default:
        return -1;
    }
  }

  static constexpr float GRAVITY_MS2 = 9.80665;
  static constexpr float ACCEL_SCALE_MODIFIER_2G = 16384.0;
  static constexpr float ACCEL_SCALE_MODIFIER_4G = 8192.0;
  static constexpr float ACCEL_SCALE_MODIFIER_8G = 4096.0;
  static constexpr float ACCEL_SCALE_MODIFIER_16G = 2048.0;

  static constexpr float GYRO_SCALE_MODIFIER_250DEG = 131.0;
  static constexpr float GYRO_SCALE_MODIFIER_500DEG = 65.5;
  static constexpr float GYRO_SCALE_MODIFIER_1000DEG = 32.8;
  static constexpr float GYRO_SCALE_MODIFIER_2000DEG = 16.4;

  static constexpr std::uint8_t ACCEL_RANGE_2G = 0x00;
  static constexpr std::uint8_t ACCEL_RANGE_4G = 0x08;
  static constexpr std::uint8_t ACCEL_RANGE_8G = 0x10;
  static constexpr std::uint8_t ACCEL_RANGE_16G = 0x18;

  static constexpr std::uint8_t GYRO_RANGE_250DEG = 0x00;
  static constexpr std::uint8_t GYRO_RANGE_500DEG = 0x08;
  static constexpr std::uint8_t GYRO_RANGE_1000DEG = 0x10;
  static constexpr std::uint8_t GYRO_RANGE_2000DEG = 0x18;

  static constexpr std::uint8_t PWR_MGMT_1 = 0x6B;
  static constexpr std::uint8_t PWR_MGMT_2 = 0x6C;

  static constexpr std::uint8_t ACCEL_XOUT0 = 0x3B;
  static constexpr std::uint8_t ACCEL_YOUT0 = 0x3D;
  static constexpr std::uint8_t ACCEL_ZOUT0 = 0x3F;

  static constexpr std::uint8_t TEMP_OUT0 = 0x41;

  static constexpr std::uint8_t GYRO_XOUT0 = 0x43;
  static constexpr std::uint8_t GYRO_YOUT0 = 0x45;
  static constexpr std::uint8_t GYRO_ZOUT0 = 0x47;

  static constexpr std::uint8_t ACCEL_CONFIG = 0x1C;
  static constexpr std::uint8_t GYRO_CONFIG = 0x1B;

  interface::i2c interface;
};
};



