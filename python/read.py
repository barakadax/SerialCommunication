import serial

rec_port = '/dev/pts/6'

data = b''
with serial.Serial(rec_port, baudrate=115_200, timeout=0.1) as rec_ser:  # exclusive=True
    while True:
        chunk = rec_ser.read(1)
        if chunk:
            data += chunk
        if not chunk and data:
            res = data.decode('utf-16')
            print(f'len of message: {len(res)}\ncontent: {res}')
            data = b''
