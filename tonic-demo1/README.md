# Tonic demo

```
brew install grpcurl

grpcurl -plaintext -proto ./proto/calculator.proto \
-d '{"a" : 2, "b" : 3}' \
'[::1]:50051' calculator.calculator.Add
```

After reflection

```
grpcurl -plaintext -d '{"a" : 2, "b" : 3}' '[::1]:50051' calculator.calculator.Add

grpcurl -plaintext '[::1]:50051' list

grpcurl -emit-defaults -plaintext '[::1]:50051' calculator.Admin.GetRequestCount
```
