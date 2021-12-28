use std::io::Read;

fn main() {
    pretty_env_logger::init();

    let access_key = fuzzy_winner::env::required_string("ACCESS_KEY");
    let secret_key = fuzzy_winner::env::required_string("SECRET_KEY");

    let client =
        fuzzy_winner::minio::Client::new("http://127.0.0.1:9000", &access_key, &secret_key)
            .unwrap();

    let mut f = std::fs::File::open("Cargo.toml").expect("open file");
    let mut buf = Vec::new();
    f.read_to_end(&mut buf).expect("read file");

    client.put_object("/sample", &buf).expect("put object");

    let result = client.get_object("/sample").expect("get object");
    println!("{:?}", result);
}
