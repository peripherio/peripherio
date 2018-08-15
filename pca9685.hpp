#include <unistd.h>
#include <utility>
#include <optional>
#include <bitset>

#include "i2c.hpp"

namespace rami::driver {

class pca9685 {
public:
  static std::optional<std::uint8_t> detect(std::uint8_t bus) {
    return interface::i2c::find(bus, [](interface::i2c&& i2c) {
        return i2c.get_address() == 0x40;
    });
  }

  pca9685(std::uint8_t bus, std::uint8_t address) : interface(interface::i2c(bus, address)) {
    /* this->interface.write(this->PWR_MGMT_1, 0x00); */
  }

private:
  interface::i2c interface;
};
};



