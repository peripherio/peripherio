# peripherio

The peripheral interface abstraction

```python
from peripherio import connect
import time

with connect() as conn:
    device = conn.find_device('gyro', {'if.type': 'i2c'})[0]
    while True:
        print(device.get_gyro())
        time.sleep(1)
```

## Getting started

First, Launch the peripherio server:

```bash
# Start the server
cargo run --bin peripherio
```

Then, you can connect to the server. For example:

```bash
cd pyperipherio
pipenv install

# Run the client
pipenv run python examples/simple_get_gyro.py
```
