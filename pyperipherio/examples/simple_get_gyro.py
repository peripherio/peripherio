from peripherio import connect
import time

with connect() as conn:
    device = conn.find_device('gyro', {'if.type': 'i2c'})[0]
    while True:
        print(device.get_gyro())
        time.sleep(1)
