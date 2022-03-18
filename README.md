# Nginx benchmark
wrk -c 100 -t 12  http://localhost:81/httptest/jquery-1.9.1.js
Running 10s test @ http://localhost:81/httptest/jquery-1.9.1.js
  12 threads and 100 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     4.29ms    4.10ms  38.67ms   85.06%
    Req/Sec     1.57k   471.89     3.38k    72.26%
  187961 requests in 10.07s, 47.03GB read
  Socket errors: connect 0, read 0, write 0, timeout 78
`Requests/sec:  18672.45`
`Transfer/sec:      4.67GB`

# Rust benchmark

wrk -c 100 -t 12  http://localhost/httptest/jquery-1.9.1.js
Running 10s test @ http://localhost/httptest/jquery-1.9.1.js
  12 threads and 100 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     4.76ms    2.92ms  43.92ms   84.61%
    Req/Sec     0.95k   199.64     1.47k    75.04%
  114440 requests in 10.10s, 28.62GB read
`Requests/sec:  11332.18`
`Transfer/sec:      2.83GB`
