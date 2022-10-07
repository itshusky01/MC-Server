import socket

s = socket.socket()
s.bind(('0.0.0.0', 25565))
s.listen()
conn, addr = s.accept()
while True:
    data = conn.recv(1024)
    if data == b'':
        break
    print(data)