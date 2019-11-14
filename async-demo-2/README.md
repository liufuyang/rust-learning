# Async IO Demo 2

Given a list of 1000 US cities, do http get request on their wiki page,
then do word counting then sum the results, group by words.

List of cities are found from [here](https://gist.github.com/Miserlou/11500b2345d3fe850c92)

Demo output (run in the root folder):
```
$ cargo run -p async-demo-2 --release

[async-io-thread-1] - 288 - Page got for city: South_Bend, character size 403551
[cpu-intensive-thread-1]: Async analysis and send data content 288...
[async-io-thread-2] - 194 - Page got for city: Sterling_Heights, character size 148136
[cpu-intensive-thread-7]: Async analysis and send data content 194...
[async-io-thread-1] - 245 - Page got for city: Westminster, character size 178566
[cpu-intensive-thread-0]: Async analysis and send data content 245...
[async-io-thread-0] - 195 - Page got for city: New_Haven, character size 644080
[cpu-intensive-thread-8]: Async analysis and send data content 195...
[async-io-thread-0] - 198 - Page got for city: Thousand_Oaks, character size 634845
[cpu-intensive-thread-4]: Async analysis and send data content 198...
[async-io-thread-1] - 478 - Page got for city: Passaic, character size 373313
[cpu-intensive-thread-6]: Async analysis and send data content 478...
[async-io-thread-1] - 123 - Page got for city: Grand_Rapids, character size 511126
[cpu-intensive-thread-2]: Async analysis and send data content 123...
[async-io-thread-2] - 45 - Page got for city: Oakland, character size 660397
[cpu-intensive-thread-9]: Async analysis and send data content 45...
[async-io-thread-2] - 386 - Page got for city: Buena_Park, character size 174010
[cpu-intensive-thread-3]: Async analysis and send data content 386...
[async-io-thread-0] - 110 - Page got for city: Columbus, character size 58672
[cpu-intensive-thread-5]: Async analysis and send data content 110...
number of maps: 1000
number of unique words: 228675
```