# A GNU wc clone in Rust
rust_wc is a wc clone written in rust. This project was a way for me to learn some Rust and play around with the language. I also wanted to see if my rust implementation could perform as good as or better than GNU wc.
Be aware; it seems it does :)

```bash
Words only:
System wc:
 10000018 test_file.txt
wc -w test_file.txt  1.64s user 0.08s system 99% cpu 1.729 total
Local rust_wc:
10000018 test_file.txt
./rust_wc -w test_file.txt  1.28s user 0.14s system 86% cpu 1.641 total
Lines only:
System wc:
 10000018 test_file.txt
wc -l test_file.txt  0.39s user 0.07s system 99% cpu 0.462 total
Local rust_wc:
10000018 test_file.txt
./rust_wc -l test_file.txt  0.27s user 0.13s system 99% cpu 0.396 total
Bytes only:
System wc:
 1000001800 test_file.txt
wc -c test_file.txt  0.00s user 0.00s system 60% cpu 0.002 total
Local rust_wc:
1000001800 test_file.txt
./rust_wc -c test_file.txt  0.00s user 0.00s system 60% cpu 0.002 total
Words and Lines:
System wc:
 10000018 10000018 test_file.txt
wc -wl test_file.txt  1.64s user 0.08s system 99% cpu 1.721 total
Local rust_wc:
10000018 10000018 test_file.txt
./rust_wc -wl test_file.txt  1.28s user 0.14s system 99% cpu 1.417 total
Words and Bytes:
System wc:
 10000018 1000001800 test_file.txt
wc -wc test_file.txt  1.64s user 0.08s system 99% cpu 1.722 total
Local rust_wc:
10000018 1000001800 test_file.txt
./rust_wc -wc test_file.txt  1.28s user 0.13s system 99% cpu 1.412 total
Lines and Bytes:
System wc:
 10000018 1000001800 test_file.txt
wc -lc test_file.txt  0.39s user 0.07s system 99% cpu 0.461 total
Local rust_wc:
10000018 1000001800 test_file.txt
./rust_wc -lc test_file.txt  0.27s user 0.12s system 99% cpu 0.394 total
Words, Lines, and Bytes:
System wc:
 10000018 10000018 1000001800 test_file.txt
wc -wlc test_file.txt  1.64s user 0.08s system 99% cpu 1.723 total
Local rust_wc:
10000018 10000018 1000001800 test_file.txt
./rust_wc -wlc test_file.txt  1.28s user 0.14s system 99% cpu 1.418 total
``````