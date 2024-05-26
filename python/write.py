import serial

sender_port = '/dev/pts/1'

with serial.Serial(sender_port, baudrate=115_200, exclusive=False) as ser:
    while True:
        msg = input('Input: ')
        ser.write(msg.encode('utf-16'))
        ser.flush()
