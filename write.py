import serial

sender_port = '/dev/pts/6'

with serial.Serial(sender_port, baudrate=115_200, exclusive=True) as ser:
    while True:
        msg = input('Input: ')
        ser.write(msg.encode('utf-16'))
        ser.flush()
