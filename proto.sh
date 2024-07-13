protoc --go_out=./go/pb/ --go_opt=paths=source_relative \
	--go-grpc_out=./go/pb/ --go-grpc_opt=paths=source_relative \
  -Iproto/ \
  $(find proto/ -iname "*.proto")

protoc --dart_out=grpc:./dart/lib/pb/ \
  -Iproto/ \
  $(find proto/ -iname "*.proto")