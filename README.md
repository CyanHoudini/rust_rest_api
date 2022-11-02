# rust_rest_api

To run install rust toolchain (```curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh```)  
Install dependicies with ```cargo build```  
Run with ```cargo run```  
Example PUT request with curl : ```curl -X PATCH --header 'Content-Type: application/json' -d '{"amount":1}' '127.0.0.1:8080/change-amount/3'```  
Example GET request with curl : ```curl GET  127.0.0.1:8080/waren```  
