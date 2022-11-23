import socket

HOST = "127.0.0.1"
PORT = 2000

# AF_INET: socket type, ipv4 (could be ipv6 or unix socket)
# SOCK_STREAM: sets to TCP
with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
    s.bind((HOST, PORT))
    s.listen()
    while True:
        conn, addr = s.accept()
        print(f"connected to {addr}")
        with conn:
            while True:
                data = conn.recv(1024)
                if not data:
                    break;
                conn.sendall(data)
