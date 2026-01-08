import subprocess
import re
import time
import sys
import os
import signal

def run_socat():
    print("Starting socat...")
    proc = subprocess.Popen(
        ["socat", "-d", "-d", "pty,raw,echo=0", "pty,raw,echo=0"],
        stderr=subprocess.PIPE,
        text=True
    )
    
    ptys = []
    while len(ptys) < 2:
        line = proc.stderr.readline()
        if not line:
            break
        print(f"socat: {line.strip()}")
        match = re.search(r"PTY is (/dev/pts/\d+)", line)
        if match:
            ptys.append(match.group(1))
            
    if len(ptys) < 2:
        proc.terminate()
        raise Exception("Failed to get PTYs from socat")
        
    return proc, ptys[0], ptys[1]

def test_pair(reader_cmd, writer_cmd, pty_reader, pty_writer, reader_label, writer_label):
    r_cwd = "."
    w_cwd = "."
    if "python/" in reader_cmd[-1]: r_cwd = "python"
    if "python/" in writer_cmd[-1]: w_cwd = "python"
    
    r_cmd = [c.replace("python/", "") if r_cwd == "python" else c for c in reader_cmd]
    w_cmd = [c.replace("python/", "") if w_cwd == "python" else c for c in writer_cmd]

    reader_proc = subprocess.Popen(
        r_cmd + [pty_reader],
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
        bufsize=1,
        cwd=r_cwd
    )
    
    time.sleep(0.25)
    
    writer_proc = subprocess.Popen(
        w_cmd + [pty_writer],
        stdin=subprocess.PIPE,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
        bufsize=1,
        cwd=w_cwd
    )
    
    message_results = []
    test_messages = ["hello world", "שלום עולם", "Very long message to test if it will work or not"]
    try:
        for msg in test_messages:
            writer_proc.stdin.write(msg + "\n")
            writer_proc.stdin.flush()
            time.sleep(0.25)
            
        time.sleep(0.5)
    finally:
        writer_proc.terminate()
        reader_proc.terminate()
        
        try:
            writer_proc.wait(timeout=2)
            reader_out, reader_err = reader_proc.communicate(timeout=2)
        except subprocess.TimeoutExpired:
            writer_proc.kill()
            reader_proc.kill()
            reader_out, reader_err = reader_proc.communicate()
        
        for msg in test_messages:
            received = msg in reader_out
            message_results.append((msg, received))
            if not received:
                print(f"FAILED: Reader='{reader_label}', Writer='{writer_label}', Message='{msg}'")
                
    return message_results

def main():
    socat_proc, pty1, pty2 = run_socat()
    print(f"PTYs: {pty1}, {pty2}")
    
    readers = [
        ["./cpp/read_sync"],
        ["./cpp/read_async"],
        ["python3", "-u", "python/read.py"],
        ["python3", "-u", "python/readAsync.py"],
        ["cargo", "run", "--quiet", "--manifest-path", "rust/Cargo.toml", "--bin", "read_sync", "--"],
        ["cargo", "run", "--quiet", "--manifest-path", "rust/Cargo.toml", "--bin", "read_async", "--"],
    ]
    
    writers = [
        ["./cpp/write_sync"],
        ["./cpp/write_async"],
        ["python3", "python/write.py"],
        ["python3", "python/writeAsync.py"],
        ["cargo", "run", "--quiet", "--manifest-path", "rust/Cargo.toml", "--bin", "write_sync", "--"],
        ["cargo", "run", "--quiet", "--manifest-path", "rust/Cargo.toml", "--bin", "write_async", "--"],
    ]
    
    results = []
    
    try:
        for r_cmd in readers:
            if "cargo" in r_cmd:
                r_label = f"rust {r_cmd[6]}"
            elif r_cmd[0].startswith("python"):
                r_label = f"python {r_cmd[-1]}"
            else:
                r_label = f"cpp {r_cmd[0].split('/')[-1]}"
                
            for w_cmd in writers:
                if "cargo" in w_cmd:
                    w_label = f"rust {w_cmd[6]}"
                elif w_cmd[0].startswith("python"):
                    w_label = f"python {w_cmd[-1]}"
                else:
                    w_label = f"cpp {w_cmd[0].split('/')[-1]}"
                
                msg_results = test_pair(r_cmd, w_cmd, pty1, pty2, r_label, w_label)
                for msg, success in msg_results:
                    results.append((r_label, w_label, msg, success))
                time.sleep(0.25)
    finally:
        socat_proc.terminate()
        
    print(f"\n--- TEST RESULTS ---\n{'Reader':<30} | {'Writer':<30} | {'Result':<10} | {'Message':<35} ")
    print("-" * 120)
    for r, w, m, res in results:
        status = "PASS" if res else "FAIL"
        m_display = (m[:32] + '...') if len(m) > 35 else m
        print(f"{r:<30} | {w:<30} | {status:<10} | {m_display:<35}")

if __name__ == "__main__":
    main()
