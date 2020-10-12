# FasterJpath

This is hopefully a faster implementation of jmespath ruby gem. It wraps over the Rust implementation of [JMESpath](https://github.com/jmespath/jmespath.rb/).

## Usage

```
pattern = "message"
json_str = '{"message":{"body": "Hello World", "favorite_numbers": [42,0, 3.14159]}}'
JPath.extract_from_string(pattern, json_str) # => {"body" => "Hello World", "favorite_numbers" => [42, 0, 3.14159]}
```

## Performance

This was a good educational experience. It turns out that `serde-json` is still slower that actual `json` parsing in Ruby. I will invest more time in using the faster JSON crate but this requires fixes to the `Variable` struct at JMESPath. 

## License

The gem is available as open source under the terms of the [MIT License](https://opensource.org/licenses/MIT).

## Code of Conduct

Everyone interacting in the FasterJpath project's codebases, issue trackers, chat rooms and mailing lists is expected to follow the [code of conduct](https://github.com/tarkalabs/faster_jpath/blob/master/CODE_OF_CONDUCT.md).
