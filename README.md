# multithreaded_prime_searcher

I'm still getting comfortable with Rust, so I wanted an opportunity to practice working with multi-threading.

I have an M2 (6 performance and 4 efficiency cores). I found that the thread number had a variable influence on program perfomance. The optimal number of threads to find all primes below 10,000,000 was 2. For 100,000,000, it was 3.