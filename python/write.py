import sys
import serial

if len(sys.argv) < 2:
    print(f"Usage: {sys.argv[0]} <port>")
    sys.exit(1)

sender_port = sys.argv[1]

with serial.Serial(sender_port, baudrate=115_200, exclusive=False) as ser:
    while True:
        msg = input('Input: ')
        ser.write(msg.encode('utf-16'))
        ser.flush()
