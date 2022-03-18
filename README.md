# Func test
```
python3 httptest.py 
test_directory_index (__main__.HttpServer)
directory index file exists ... ok
test_document_root_escaping (__main__.HttpServer)
document root escaping forbidden ... ok
test_empty_request (__main__.HttpServer)
Send empty line ... ok
test_file_in_nested_folders (__main__.HttpServer)
file located in nested folders ... ok
test_file_not_found (__main__.HttpServer)
absent file returns 404 ... ok
test_file_type_css (__main__.HttpServer)
Content-Type for .css ... ok
test_file_type_gif (__main__.HttpServer)
Content-Type for .gif ... ok
test_file_type_html (__main__.HttpServer)
Content-Type for .html ... ok
test_file_type_jpeg (__main__.HttpServer)
Content-Type for .jpeg ... ok
test_file_type_jpg (__main__.HttpServer)
Content-Type for .jpg ... ok
test_file_type_js (__main__.HttpServer)
Content-Type for .js ... ok
test_file_type_png (__main__.HttpServer)
Content-Type for .png ... ok
test_file_type_swf (__main__.HttpServer)
Content-Type for .swf ... ok
test_file_urlencoded (__main__.HttpServer)
urlencoded filename ... ok
test_file_with_dot_in_name (__main__.HttpServer)
file with two dots in name ... ok
test_file_with_query_string (__main__.HttpServer)
query string with get params ... ok
test_file_with_slash_after_filename (__main__.HttpServer)
slash after filename ... ok
test_file_with_spaces (__main__.HttpServer)
filename with spaces ... ok
test_head_method (__main__.HttpServer)
head method support ... ok
test_index_not_found (__main__.HttpServer)
directory index file absent ... ok
test_large_file (__main__.HttpServer)
large file downloaded correctly ... ok
test_post_method (__main__.HttpServer)
post method forbidden ... ok
test_request_without_two_newlines (__main__.HttpServer)
Send GET without to newlines ... ok
test_server_header (__main__.HttpServer)
Server header exists ... ok

----------------------------------------------------------------------
Ran 24 tests in 0.025s

OK
```

# Nginx benchmark

```
wrk -c 100 -t 12  http://localhost:81/httptest/jquery-1.9.1.js
Running 10s test @ http://localhost:81/httptest/jquery-1.9.1.js
  12 threads and 100 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     4.29ms    4.10ms  38.67ms   85.06%
    Req/Sec     1.57k   471.89     3.38k    72.26%
  187961 requests in 10.07s, 47.03GB read
  Socket errors: connect 0, read 0, write 0, timeout 78
Requests/sec:  18672.45
Transfer/sec:      4.67GB
```

# Rust benchmark

```
wrk -c 100 -t 12  http://localhost/httptest/jquery-1.9.1.js
Running 10s test @ http://localhost/httptest/jquery-1.9.1.js
  12 threads and 100 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     4.76ms    2.92ms  43.92ms   84.61%
    Req/Sec     0.95k   199.64     1.47k    75.04%
  114440 requests in 10.10s, 28.62GB read
Requests/sec:  11332.18
Transfer/sec:      2.83GB
```