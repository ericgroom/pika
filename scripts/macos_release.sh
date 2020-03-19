cargo build --release
mkdir artifacts
cp ./target/release/pika_cli ./artifacts/pika
pushd ./artifacts
tar -cvzf darwin.tar.gz pika
popd
mv ./artifacts/darwin.tar.gz .
rm -rf artifacts